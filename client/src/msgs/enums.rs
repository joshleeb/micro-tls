macro_rules! msg_enum {
    (
        $ident: ident, $ty: ty;
        { $($var: ident => $val: expr),*$(,)? }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $ident {
            $($var),*,
            Unknown($ty),
        }

        impl From<$ident> for $ty {
            fn from(val: $ident) -> Self {
                match val {
                    $($ident::$var => $val),*,
                    $ident::Unknown(x) => x,
                }
            }
        }

        impl From<$ty> for $ident {
            fn from(val: $ty) -> Self {
                match val {
                    $($val => $ident::$var),*,
                    x => $ident::Unknown(x),
                }
            }
        }

        impl<'a> crate::msgs::Codec<'a> for $ident {
            fn encode(&self, enc: &mut crate::msgs::Encoder<'a>) {
                <$ty>::from(*self).encode(enc);
            }

            fn decode(dec: &mut crate::msgs::Decoder<'a>) -> Option<Self> {
                <$ty>::decode(dec).map(|item| {
                    match item {
                        $($val => $ident::$var),*,
                        x => $ident::Unknown(x),
                    }
                })
            }
        }

        impl<'a> crate::msgs::CodecLength<'a> for $ident {
            const LENGTH: usize = <$ty>::LENGTH;

            fn encode_len(len: usize, enc: &mut crate::msgs::Encoder<'a>) {
                <$ty>::encode_len(len, enc);
            }

            fn decode_len(dec: &mut crate::msgs::Decoder<'a>) -> Option<usize> {
                <$ty>::decode_len(dec)
            }
        }
    };
}

msg_enum! {
    ProtocolVersion, u16;
    {
        TLSv1_2 => 0x0303,
        TLSv1_3 => 0x0304,
    }
}

msg_enum! {
    CompressionMethod, u8;
    {
        Null => 0x00,
        Deflate => 0x01,
        LSZ => 0x40
    }
}

msg_enum! {
    CipherSuite, u16;
    {
        TlsAes128GcmSha256 => 0x1301,
        TlsAes256GcmSha384 => 0x1302,
        TlsChaCha20Poly1305Sha256 => 0x1303,
    }
}

msg_enum! {
    ExtensionType, u16;
    {
        SignatureAlgorithms => 0x000d,
        SupportedVersions => 0x002b,
    }
}

msg_enum! {
    SignatureScheme, u16;
    {
        // RSASSA-PKCS1-v1_5 algorithms.
        RSA_PKCS1_SHA256 => 0x0401,
        RSA_PKCS1_SHA384 => 0x0501,
        RSA_PKCS1_SHA512 => 0x0601,

        // ECDSA algorithms.
        ECDSA_NISTP256_SHA256 => 0x0403,
        ECDSA_NISTP384_SHA384 => 0x0503,
        ECDSA_NISTP521_SHA512 => 0x0603,

        // RSASSA-PSS algorithms with public key OID rsaEncryption.
        RSA_PSS_SHA256 => 0x0804,
        RSA_PSS_SHA384 => 0x0805,
        RSA_PSS_SHA512 => 0x0806,

        // EdDSA algorithms.
        ED25519 => 0x0807,
        ED448 => 0x0808,

        // Legacy algorithms.
        RSA_PKCS1_SHA1 => 0x0201,
        ECDSA_SHA1_Legacy => 0x0203,
    }
}
