// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`CreateBucket`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`acl(BucketCannedAcl)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::acl) / [`set_acl(Option<BucketCannedAcl>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::set_acl):<br>required: **false**<br><p>The canned ACL to apply to the bucket.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note><br>
    ///   - [`bucket(impl Into<String>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::bucket) / [`set_bucket(Option<String>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::set_bucket):<br>required: **true**<br><p>The name of the bucket to create.</p>  <p> <b>General purpose buckets</b> - For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/bucketnamingrules.html">Bucket naming rules</a> in the <i>Amazon S3 User Guide</i>.</p>  <p> <b>Directory buckets </b> - When you use this operation with a directory bucket, you must use path-style requests in the format <code>https://s3express-control.<i>region_code</i>.amazonaws.com/<i>bucket-name</i> </code>. Virtual-hosted-style requests aren't supported. Directory bucket names must be unique in the chosen Availability Zone. Bucket names must also follow the format <code> <i>bucket_base_name</i>--<i>az_id</i>--x-s3</code> (for example, <code> <i>DOC-EXAMPLE-BUCKET</i>--<i>usw2-az2</i>--x-s3</code>). For information about bucket naming restrictions, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/directory-bucket-naming-rules.html">Directory bucket naming rules</a> in the <i>Amazon S3 User Guide</i> </p><br>
    ///   - [`create_bucket_configuration(CreateBucketConfiguration)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::create_bucket_configuration) / [`set_create_bucket_configuration(Option<CreateBucketConfiguration>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::set_create_bucket_configuration):<br>required: **false**<br><p>The configuration information for the bucket.</p><br>
    ///   - [`grant_full_control(impl Into<String>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::grant_full_control) / [`set_grant_full_control(Option<String>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::set_grant_full_control):<br>required: **false**<br><p>Allows grantee the read, write, read ACP, and write ACP permissions on the bucket.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note><br>
    ///   - [`grant_read(impl Into<String>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::grant_read) / [`set_grant_read(Option<String>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::set_grant_read):<br>required: **false**<br><p>Allows grantee to list the objects in the bucket.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note><br>
    ///   - [`grant_read_acp(impl Into<String>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::grant_read_acp) / [`set_grant_read_acp(Option<String>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::set_grant_read_acp):<br>required: **false**<br><p>Allows grantee to read the bucket ACL.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note><br>
    ///   - [`grant_write(impl Into<String>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::grant_write) / [`set_grant_write(Option<String>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::set_grant_write):<br>required: **false**<br><p>Allows grantee to create new objects in the bucket.</p>  <p>For the bucket and object owners of existing objects, also allows deletions and overwrites of those objects.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note><br>
    ///   - [`grant_write_acp(impl Into<String>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::grant_write_acp) / [`set_grant_write_acp(Option<String>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::set_grant_write_acp):<br>required: **false**<br><p>Allows grantee to write the ACL for the applicable bucket.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note><br>
    ///   - [`object_lock_enabled_for_bucket(bool)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::object_lock_enabled_for_bucket) / [`set_object_lock_enabled_for_bucket(Option<bool>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::set_object_lock_enabled_for_bucket):<br>required: **false**<br><p>Specifies whether you want S3 Object Lock to be enabled for the new bucket.</p> <note>   <p>This functionality is not supported for directory buckets.</p>  </note><br>
    ///   - [`object_ownership(ObjectOwnership)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::object_ownership) / [`set_object_ownership(Option<ObjectOwnership>)`](crate::operation::create_bucket::builders::CreateBucketFluentBuilder::set_object_ownership):<br>required: **false**<br><p>The container element for object ownership for a bucket's ownership controls.</p>  <p> <code>BucketOwnerPreferred</code> - Objects uploaded to the bucket change ownership to the bucket owner if the objects are uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>  <p> <code>ObjectWriter</code> - The uploading account will own the object if the object is uploaded with the <code>bucket-owner-full-control</code> canned ACL.</p>  <p> <code>BucketOwnerEnforced</code> - Access control lists (ACLs) are disabled and no longer affect permissions. The bucket owner automatically owns and has full control over every object in the bucket. The bucket only accepts PUT requests that don't specify an ACL or specify bucket owner full control ACLs (such as the predefined <code>bucket-owner-full-control</code> canned ACL or a custom ACL in XML format that grants the same permissions).</p>  <p>By default, <code>ObjectOwnership</code> is set to <code>BucketOwnerEnforced</code> and ACLs are disabled. We recommend keeping ACLs disabled, except in uncommon use cases where you must control access for each object individually. For more information about S3 Object Ownership, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/userguide/about-object-ownership.html">Controlling ownership of objects and disabling ACLs for your bucket</a> in the <i>Amazon S3 User Guide</i>. </p> <note>   <p>This functionality is not supported for directory buckets. Directory buckets use the bucket owner enforced setting for S3 Object Ownership.</p>  </note><br>
    /// - On success, responds with [`CreateBucketOutput`](crate::operation::create_bucket::CreateBucketOutput) with field(s):
    ///   - [`location(Option<String>)`](crate::operation::create_bucket::CreateBucketOutput::location): <p>A forward slash followed by the name of the bucket.</p>
    /// - On failure, responds with [`SdkError<CreateBucketError>`](crate::operation::create_bucket::CreateBucketError)
    pub fn create_bucket(&self) -> crate::operation::create_bucket::builders::CreateBucketFluentBuilder {
        crate::operation::create_bucket::builders::CreateBucketFluentBuilder::new(self.handle.clone())
    }
}
