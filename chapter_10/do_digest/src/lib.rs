#[macro_use]
extern crate do_digest_derive;
extern crate sha2;

use sha2::Sha256;

extern crate digest;
use digest::Digest;

pub trait DoDigest<D: Digest> {
    fn update_digest(&self, &mut D);
}

impl<D> DoDigest<D> for String
    where D: Digest
{
    fn update_digest(&self, digest: &mut D) {
        digest.input(self.as_bytes());
    }
}

impl<D> DoDigest<D> for &'static str
    where D: Digest
{
    fn update_digest(&self, digest: &mut D) {
        digest.input(self.as_bytes());
    }
}

impl<D, X> DoDigest<D> for [X]
    where D: Digest,
          X: DoDigest<D>,
{
    fn update_digest(&self, digest: &mut D) {
        for x in self {
            x.update_digest(digest);
        }
    }
}

impl<D, X> DoDigest<D> for Vec<X>
    where D: Digest,
          X: DoDigest<D>,
{
    fn update_digest(&self, digest: &mut D) {
        for x in self {
            x.update_digest(digest);
        }
    }
}

pub fn compute_digest<X: DoDigest<Sha256>>(val: &X) -> String {
    let mut hasher = Sha256::default();
    val.update_digest(&mut hasher);
    format!("{:x}", hasher.result())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(DoDigest)]
    pub struct Foo {
        bar: &'static str,
        baz: &'static str,
        quux: Vec<&'static str>,
    }

    #[test]
    fn testme() {
        let test = Foo { bar: "Bar", baz: "Baz", quux: vec!["1", "2", "3"] };
        println!("{}", compute_digest(&test));
    }
}
