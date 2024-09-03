//! This file has been automatically generated by `objc2`'s `header-translator`.
//! DO NOT EDIT
use objc2::__framework_prelude::*;

use crate::*;

// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NSTextCheckingType(pub u64);
bitflags::bitflags! {
    impl NSTextCheckingType: u64 {
        #[doc(alias = "NSTextCheckingTypeOrthography")]
        const Orthography = 1<<0;
        #[doc(alias = "NSTextCheckingTypeSpelling")]
        const Spelling = 1<<1;
        #[doc(alias = "NSTextCheckingTypeGrammar")]
        const Grammar = 1<<2;
        #[doc(alias = "NSTextCheckingTypeDate")]
        const Date = 1<<3;
        #[doc(alias = "NSTextCheckingTypeAddress")]
        const Address = 1<<4;
        #[doc(alias = "NSTextCheckingTypeLink")]
        const Link = 1<<5;
        #[doc(alias = "NSTextCheckingTypeQuote")]
        const Quote = 1<<6;
        #[doc(alias = "NSTextCheckingTypeDash")]
        const Dash = 1<<7;
        #[doc(alias = "NSTextCheckingTypeReplacement")]
        const Replacement = 1<<8;
        #[doc(alias = "NSTextCheckingTypeCorrection")]
        const Correction = 1<<9;
        #[doc(alias = "NSTextCheckingTypeRegularExpression")]
        const RegularExpression = 1<<10;
        #[doc(alias = "NSTextCheckingTypePhoneNumber")]
        const PhoneNumber = 1<<11;
        #[doc(alias = "NSTextCheckingTypeTransitInformation")]
        const TransitInformation = 1<<12;
    }
}

unsafe impl Encode for NSTextCheckingType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for NSTextCheckingType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

pub type NSTextCheckingTypes = u64;

pub const NSTextCheckingAllSystemTypes: NSTextCheckingTypes = 0xffffffff;
pub const NSTextCheckingAllCustomTypes: NSTextCheckingTypes = 0xffffffff << 32;
pub const NSTextCheckingAllTypes: NSTextCheckingTypes =
    NSTextCheckingAllSystemTypes | NSTextCheckingAllCustomTypes;

// NS_TYPED_EXTENSIBLE_ENUM
#[cfg(feature = "NSString")]
pub type NSTextCheckingKey = NSString;

extern_class!(
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct NSTextCheckingResult;

    unsafe impl ClassType for NSTextCheckingResult {
        type Super = NSObject;
        type Mutability = InteriorMutable;
    }
);

#[cfg(feature = "NSObject")]
unsafe impl NSCoding for NSTextCheckingResult {}

#[cfg(feature = "NSObject")]
unsafe impl NSCopying for NSTextCheckingResult {}

unsafe impl NSObjectProtocol for NSTextCheckingResult {}

#[cfg(feature = "NSObject")]
unsafe impl NSSecureCoding for NSTextCheckingResult {}

extern_methods!(
    unsafe impl NSTextCheckingResult {
        #[method(resultType)]
        pub unsafe fn resultType(&self) -> NSTextCheckingType;

        #[cfg(feature = "NSRange")]
        #[method(range)]
        pub unsafe fn range(&self) -> NSRange;
    }
);

extern_methods!(
    /// Methods declared on superclass `NSObject`
    unsafe impl NSTextCheckingResult {
        #[method_id(@__retain_semantics Init init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[method_id(@__retain_semantics New new)]
        pub unsafe fn new() -> Retained<Self>;
    }
);

extern_methods!(
    /// NSTextCheckingResultOptional
    unsafe impl NSTextCheckingResult {
        #[cfg(feature = "NSOrthography")]
        #[method_id(@__retain_semantics Other orthography)]
        pub unsafe fn orthography(&self) -> Option<Retained<NSOrthography>>;

        #[cfg(all(feature = "NSArray", feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other grammarDetails)]
        pub unsafe fn grammarDetails(
            &self,
        ) -> Option<Retained<NSArray<NSDictionary<NSString, AnyObject>>>>;

        #[cfg(feature = "NSDate")]
        #[method_id(@__retain_semantics Other date)]
        pub unsafe fn date(&self) -> Option<Retained<NSDate>>;

        #[cfg(feature = "NSTimeZone")]
        #[method_id(@__retain_semantics Other timeZone)]
        pub unsafe fn timeZone(&self) -> Option<Retained<NSTimeZone>>;

        #[cfg(feature = "NSDate")]
        #[method(duration)]
        pub unsafe fn duration(&self) -> NSTimeInterval;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other components)]
        pub unsafe fn components(
            &self,
        ) -> Option<Retained<NSDictionary<NSTextCheckingKey, NSString>>>;

        #[cfg(feature = "NSURL")]
        #[method_id(@__retain_semantics Other URL)]
        pub unsafe fn URL(&self) -> Option<Retained<NSURL>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other replacementString)]
        pub unsafe fn replacementString(&self) -> Option<Retained<NSString>>;

        #[cfg(all(feature = "NSArray", feature = "NSString"))]
        #[method_id(@__retain_semantics Other alternativeStrings)]
        pub unsafe fn alternativeStrings(&self) -> Option<Retained<NSArray<NSString>>>;

        #[cfg(feature = "NSRegularExpression")]
        #[method_id(@__retain_semantics Other regularExpression)]
        pub unsafe fn regularExpression(&self) -> Option<Retained<NSRegularExpression>>;

        #[cfg(feature = "NSString")]
        #[method_id(@__retain_semantics Other phoneNumber)]
        pub unsafe fn phoneNumber(&self) -> Option<Retained<NSString>>;

        #[method(numberOfRanges)]
        pub unsafe fn numberOfRanges(&self) -> NSUInteger;

        #[cfg(feature = "NSRange")]
        #[method(rangeAtIndex:)]
        pub unsafe fn rangeAtIndex(&self, idx: NSUInteger) -> NSRange;

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method(rangeWithName:)]
        pub unsafe fn rangeWithName(&self, name: &NSString) -> NSRange;

        #[method_id(@__retain_semantics Other resultByAdjustingRangesWithOffset:)]
        pub unsafe fn resultByAdjustingRangesWithOffset(
            &self,
            offset: NSInteger,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(all(feature = "NSDictionary", feature = "NSString"))]
        #[method_id(@__retain_semantics Other addressComponents)]
        pub unsafe fn addressComponents(
            &self,
        ) -> Option<Retained<NSDictionary<NSTextCheckingKey, NSString>>>;
    }
);

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSTextCheckingNameKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSTextCheckingJobTitleKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSTextCheckingOrganizationKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSTextCheckingStreetKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSTextCheckingCityKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSTextCheckingStateKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSTextCheckingZIPKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSTextCheckingCountryKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSTextCheckingPhoneKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSTextCheckingAirlineKey: &'static NSTextCheckingKey;
}

extern "C" {
    #[cfg(feature = "NSString")]
    pub static NSTextCheckingFlightKey: &'static NSTextCheckingKey;
}

extern_methods!(
    /// NSTextCheckingResultCreation
    unsafe impl NSTextCheckingResult {
        #[cfg(all(feature = "NSOrthography", feature = "NSRange"))]
        #[method_id(@__retain_semantics Other orthographyCheckingResultWithRange:orthography:)]
        pub unsafe fn orthographyCheckingResultWithRange_orthography(
            range: NSRange,
            orthography: &NSOrthography,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(feature = "NSRange")]
        #[method_id(@__retain_semantics Other spellCheckingResultWithRange:)]
        pub unsafe fn spellCheckingResultWithRange(
            range: NSRange,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(all(
            feature = "NSArray",
            feature = "NSDictionary",
            feature = "NSRange",
            feature = "NSString"
        ))]
        #[method_id(@__retain_semantics Other grammarCheckingResultWithRange:details:)]
        pub unsafe fn grammarCheckingResultWithRange_details(
            range: NSRange,
            details: &NSArray<NSDictionary<NSString, AnyObject>>,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(all(feature = "NSDate", feature = "NSRange"))]
        #[method_id(@__retain_semantics Other dateCheckingResultWithRange:date:)]
        pub unsafe fn dateCheckingResultWithRange_date(
            range: NSRange,
            date: &NSDate,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(all(feature = "NSDate", feature = "NSRange", feature = "NSTimeZone"))]
        #[method_id(@__retain_semantics Other dateCheckingResultWithRange:date:timeZone:duration:)]
        pub unsafe fn dateCheckingResultWithRange_date_timeZone_duration(
            range: NSRange,
            date: &NSDate,
            time_zone: &NSTimeZone,
            duration: NSTimeInterval,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(all(feature = "NSDictionary", feature = "NSRange", feature = "NSString"))]
        #[method_id(@__retain_semantics Other addressCheckingResultWithRange:components:)]
        pub unsafe fn addressCheckingResultWithRange_components(
            range: NSRange,
            components: &NSDictionary<NSTextCheckingKey, NSString>,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(all(feature = "NSRange", feature = "NSURL"))]
        #[method_id(@__retain_semantics Other linkCheckingResultWithRange:URL:)]
        pub unsafe fn linkCheckingResultWithRange_URL(
            range: NSRange,
            url: &NSURL,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method_id(@__retain_semantics Other quoteCheckingResultWithRange:replacementString:)]
        pub unsafe fn quoteCheckingResultWithRange_replacementString(
            range: NSRange,
            replacement_string: &NSString,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method_id(@__retain_semantics Other dashCheckingResultWithRange:replacementString:)]
        pub unsafe fn dashCheckingResultWithRange_replacementString(
            range: NSRange,
            replacement_string: &NSString,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method_id(@__retain_semantics Other replacementCheckingResultWithRange:replacementString:)]
        pub unsafe fn replacementCheckingResultWithRange_replacementString(
            range: NSRange,
            replacement_string: &NSString,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method_id(@__retain_semantics Other correctionCheckingResultWithRange:replacementString:)]
        pub unsafe fn correctionCheckingResultWithRange_replacementString(
            range: NSRange,
            replacement_string: &NSString,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(all(feature = "NSArray", feature = "NSRange", feature = "NSString"))]
        #[method_id(@__retain_semantics Other correctionCheckingResultWithRange:replacementString:alternativeStrings:)]
        pub unsafe fn correctionCheckingResultWithRange_replacementString_alternativeStrings(
            range: NSRange,
            replacement_string: &NSString,
            alternative_strings: &NSArray<NSString>,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(all(feature = "NSRange", feature = "NSRegularExpression"))]
        #[method_id(@__retain_semantics Other regularExpressionCheckingResultWithRanges:count:regularExpression:)]
        pub unsafe fn regularExpressionCheckingResultWithRanges_count_regularExpression(
            ranges: NSRangePointer,
            count: NSUInteger,
            regular_expression: &NSRegularExpression,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(all(feature = "NSRange", feature = "NSString"))]
        #[method_id(@__retain_semantics Other phoneNumberCheckingResultWithRange:phoneNumber:)]
        pub unsafe fn phoneNumberCheckingResultWithRange_phoneNumber(
            range: NSRange,
            phone_number: &NSString,
        ) -> Retained<NSTextCheckingResult>;

        #[cfg(all(feature = "NSDictionary", feature = "NSRange", feature = "NSString"))]
        #[method_id(@__retain_semantics Other transitInformationCheckingResultWithRange:components:)]
        pub unsafe fn transitInformationCheckingResultWithRange_components(
            range: NSRange,
            components: &NSDictionary<NSTextCheckingKey, NSString>,
        ) -> Retained<NSTextCheckingResult>;
    }
);
