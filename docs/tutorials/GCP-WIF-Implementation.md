# Google Cloud Platform Workload Identity Federation Support
## Vector v0.54 Enhancement

---

## Summary

Implement Google Cloud Platform (GCP) Workload Identity Federation (WIF) support in /viaq/vector v0.54, enabling **keyless authentication** for all GCP integrations.

This enhancement eliminates the operational burden and security risks of managing service account keys while maintaining full backwards compatibility with existing deployments.

### Key Features
- ✅ Full WIF support across all GCP sinks and sources
- ✅ Backward compatible with existing authentication methods
- ✅ Production-ready and tested implementation

---

### Industry
Google Cloud **recommends** Workload Identity Federation as the preferred authentication method for workloads running outside GCP, particularly in OpenShift environments. This is becoming a **requirement** for security-conscious enterprises and is mandated by many compliance frameworks.

---

### Auth Comparison

**Traditional Auth (Static Keys - INSECURE):**
```
Collector Pod → Service Account Key (NEVER EXPIRES) → GCP APIs
                     ↑
              [Stored as secret]
              [Can be copied anywhere]
              [Persists in images/logs]
              [Indefinite access if stolen]
```

**Workload Identity Federation (Dynamic Tokens - SECURE):**
```
Collector Pod → OpenShift Pod Token → GCP Token Exchange → Short-lived Token (1hr) → GCP APIs
                  ↑                         ↓                        ↓
         [Bound to pod identity]    [Logged & audited]      [Auto-refreshed]
         [Cannot be copied]          [Traceable]             [Auto-expires]
         [Terminates with pod]       [Revocable]             [Scoped access]
```

### Security Enhancement

With WIF, there are **no long-lived credentials to protect**. Even if an attacker compromises a pod, they only gain access for the current token's remaining lifetime (maximum 1 hour).  Additionally, the token is cryptographically bound to that specific OpenShift workload identity, making it useless elsewhere.

---

### Implementation Details

**Core Authentication Module** (`src/gcp.rs`)
- Complete rewrite using official Google authentication library
- Support for multiple OAuth scopes
- Automatic token refresh handling
- Thread-safe credential management

**GCP Sinks** (Data Destinations)
- `gcp_cloud_storage` - Cloud Storage bucket output
- `gcp_pubsub` - Pub/Sub topic publishing
- `gcp_stackdriver_logs` - Cloud Logging
- `gcp_stackdriver_metrics` - Cloud Monitoring
- `gcp_chronicle` - Chronicle Security

### Cargo
```toml
# Before
goauth = "0.16.0"
smpl_jwt = "0.8.0"

# After
google-cloud-auth = "1.6"  # Official Google library with full WIF support    woot!
```



### New "type" of Credentials File
```json
{
  "type": "external_account",
  "audience": "//iam.googleapis.com/projects/PROJECT_NUM/locations/global/workloadIdentityPools/POOL/providers/PROVIDER",
  "subject_token_type": "urn:ietf:params:oauth:token-type:jwt",
  "token_url": "https://sts.googleapis.com/v1/token",
  "credential_source": {
    "file": "/var/run/ocp-collector/serviceaccount/token"
  },
  "service_account_impersonation_url": "https://iamcredentials.googleapis.com/v1/projects/-/serviceAccounts/SA@PROJECT.iam.gserviceaccount.com:generateAccessToken"
}
```

---

### Benefits

- ✅ **Zero Key Management:** No rotation schedules, no distribution pipelines, no key tracking
  - Eliminates entire category of operational incidents
  - No "emergency key rotation" fire drills
  - No secret management infrastructure needed

- ✅ **Least Privilege Access:** Tokens are scoped to specific GCP APIs and resources
  - Fine-grained IAM policies per workload
  - No over-privileged "one key for everything" scenarios

- ✅ **Complete Audit Trail:** Every authentication is logged and traceable
  - GCP Cloud Audit Logs show which OpenShift pod accessed which resource
  - Token exchange events provide forensic evidence
  - Real-time detection of unauthorized access attempts

- ✅ **Multi-Cloud Support:** Works with AWS, Azure, and other identity providers
- ✅ **Industry Best Practice:** Aligns with Google Cloud's **required** approach for modern workloads

- ✅ **Risk Reduction:** Eliminates entire class of credential leakage incidents
  - No more "key accidentally committed to GitHub" incidents
  - No credential exposure via container image scanning
  - Protection against supply chain attacks targeting secrets

---
### Testing

**Unit Tests**
```bash
✓ gcp::tests::skip_authentication
✓ gcp::tests::uses_api_key
✓ gcp::tests::fails_bad_api_key
```

**Build Verification**
```bash
✓ Compiles with all GCP features
✓ Rust 1.92 compatibility verified
✓ No clippy warnings or errors
```

**Integration Testing**
- GCP integration test suite available (`cargo vdev int test gcp`)
- Tested with service account credentials
- Verified with external account credentials (WIF)

---

## Authentication Priority

Vector searches for credentials in this order:

1. `api_key` (if configured)
2. `credentials_path` (if configured) - supports both Service Account and External Account (WIF) files
3. **Application Default Credentials (ADC)** - automatic fallback

ADC searches: `GOOGLE_APPLICATION_CREDENTIALS` env var → gcloud CLI → GCE/GKE metadata server

**Behavior Change**: When no explicit credentials are configured, Vector now attempts ADC instead of failing immediately. Tests renamed from `fails_missing_creds` to `falls_back_to_adc` to reflect this improvement.

---

## Steps

**Step 1:** Create a GCP service account to be used by the ClusterLogForwarder:
```bash
gcloud iam service-accounts create SERVICE_ACCOUNT_NAME \
  --display-name="OpenShift Logging Admin" \
  --project=PROJECT_ID
```

**Step 2:** Bind Permissions to your Collector Service Account
```bash
# Allow OpenShift service account to impersonate GCP service account
gcloud iam service-accounts add-iam-policy-binding SERVICE_ACCOUNT_NAME@project.iam.gserviceaccount.com \
  --role=roles/iam.workloadIdentityUser \
  --member="principal://iam.googleapis.com/projects/PROJECT_NUM/locations/global/workloadIdentityPools/POOL/subject/system:serviceaccount:NAMESPACE:MY_COLLECTOR_SERVICE_ACCOUNT"
```

**Step 3:** Generate a Configuration file for the External Account (Google Service Account)
```bash
gcloud iam workload-identity-pools create-cred-config \
  projects/PROJECT_NUM/locations/global/workloadIdentityPools/POOL/providers/PROVIDER \
  --service-account=SA@PROJECT.iam.gserviceaccount.com \
  --output-file=external-account.json \
  --credential-source-file=/var/run/ocp-collector/serviceaccount/token
```

**Step 4:** Create secret and configure ClusterLogForwarder

```bash
# Create secret in openshift-logging namespace
oc create secret generic gcp-wif-credentials \
  --from-file=credentials.json=external-account.json \
  -n openshift-logging
```

**ClusterLogForwarder Configuration**:
```yaml
apiVersion: observability.openshift.io/v1
kind: ClusterLogForwarder
metadata:
  name: instance
  namespace: openshift-logging
spec:
  outputs:
    - name: gcp-wif-logging
      type: googleCloudLogging
      googleCloudLogging:
        id:
          type: project
          value: my-project123
        logId: my-logs123
        authentication:
          credentials:
            secretName: gcp-wif-credentials
            key: external-account.json
...
# The external-account.json contains external account config (WIF)
# Vector detects the credential type and uses appropriate auth flow
```

## References

- **Google Cloud Documentation:** [Workload Identity Federation](https://cloud.google.com/iam/docs/workload-identity-federation)
- **google-cloud-auth Library:** [Official Rust SDK](https://github.com/googleapis/google-cloud-rust)
- **Vector Documentation:** [GCP Authentication](https://vector.dev/docs/reference/configuration/sinks/gcp_stackdriver_logs/#authentication)
- **Ticket:** RH OpenShift LOG-9171
- **Branch:** `v0.54.0-rh-gcp-wif`
