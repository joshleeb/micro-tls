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
        SupportedVersions => 0x002b,
    }
}
