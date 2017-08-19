#[macro_use]
extern crate do_digest_derive;

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

impl<D> DoDigest<D> for str
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

#[derive(DoDigest)]
pub struct Foo {
    bar: String,
    baz: String,
    quux: Vec<String>,
}
