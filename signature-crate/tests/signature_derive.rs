/// "Tests" for code generated by `signature_derive`
#[cfg(all(test, feature = "derive-preview"))]
mod tests {
    use digest::{generic_array::GenericArray, Digest};
    use hex_literal::hex;
    use sha2::Sha256;
    use signature::{
        DigestSignature, DigestSigner, DigestVerifier, Error, Signature, Signer, Verifier,
    };

    /// Test vector to compute SHA-256 digest of
    const INPUT_STRING: &[u8] = b"abc";

    /// Expected SHA-256 digest for the input string
    const INPUT_STRING_DIGEST: [u8; 32] =
        hex!("ba7816bf 8f01cfea 414140de 5dae2223 b00361a3 96177a9c b410ff61 f20015ad");

    /// Dummy signature which just contains a digest output
    #[derive(Debug)]
    struct DummySignature(GenericArray<u8, <Sha256 as Digest>::OutputSize>);

    impl Signature for DummySignature {
        fn from_bytes(bytes: impl AsRef<[u8]>) -> Result<Self, Error> {
            Ok(DummySignature(GenericArray::clone_from_slice(
                bytes.as_ref(),
            )))
        }
    }

    impl AsRef<[u8]> for DummySignature {
        fn as_ref(&self) -> &[u8] {
            self.0.as_ref()
        }
    }

    impl DigestSignature for DummySignature {
        type Digest = Sha256;
    }

    /// Dummy signer which just returns the message digest as a `DummySignature`
    #[derive(Signer, Default)]
    struct DummySigner {}

    impl DigestSigner<Sha256, DummySignature> for DummySigner {
        fn try_sign_digest(&self, digest: Sha256) -> Result<DummySignature, Error> {
            DummySignature::from_bytes(digest.result())
        }
    }

    /// Dummy verifier which ensures the `DummySignature` digest matches the
    /// expected value.
    ///
    /// Panics (via `assert_eq!`) if the value is not what is expected.
    #[derive(Verifier, Default)]
    struct DummyVerifier {}

    impl DigestVerifier<Sha256, DummySignature> for DummyVerifier {
        fn verify_digest(&self, digest: Sha256, signature: &DummySignature) -> Result<(), Error> {
            let actual_digest = digest.result();
            assert_eq!(signature.as_ref(), actual_digest.as_ref());
            Ok(())
        }
    }

    #[test]
    fn derived_signer_impl() {
        let sig: DummySignature = DummySigner::default().sign(INPUT_STRING);
        assert_eq!(sig.as_ref(), INPUT_STRING_DIGEST.as_ref())
    }

    #[test]
    fn derived_verifier_impl() {
        let sig: DummySignature = DummySigner::default().sign(INPUT_STRING);
        assert!(DummyVerifier::default().verify(INPUT_STRING, &sig).is_ok());
    }
}
