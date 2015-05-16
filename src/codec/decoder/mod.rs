pub mod video;
pub use self::video::Video;

pub mod audio;
pub use self::audio::Audio;

pub mod subtitle;
pub use self::subtitle::Subtitle;

pub mod slice;

pub mod conceal;
pub use self::conceal::*;

pub mod check;
pub use self::check::*;

use std::ffi::CString;
use std::ptr;
use std::ops::Deref;

use ffi::*;
use super::Id;
use super::context::Opened;
use ::{Codec, Error};
use ::media;

pub struct Decoder(pub Opened);

impl Decoder {
	pub fn video(self) -> Result<Video, Error> {
		if self.medium() == media::Type::Video {
			Ok(Video(self))
		}
		else {
			Err(Error::from(AVERROR_INVALIDDATA))
		}
	}

	pub fn audio(self) -> Result<Audio, Error> {
		if self.medium() == media::Type::Audio {
			Ok(Audio(self))
		}
		else {
			Err(Error::from(AVERROR_INVALIDDATA))
		}
	}

	pub fn subtitle(self) -> Result<Subtitle, Error> {
		if self.medium() == media::Type::Subtitle {
			Ok(Subtitle(self))
		}
		else {
			Err(Error::from(AVERROR_INVALIDDATA))
		}
	}

	pub fn conceal(&mut self, value: Conceal) {
		unsafe {
			(*self.ptr).error_concealment = value.bits();
		}
	}

	pub fn check(&mut self, value: Check) {
		unsafe {
			(*self.ptr).err_recognition = value.bits();
		}
	}
}

impl Deref for Decoder {
	type Target = Opened;

	fn deref(&self) -> &<Self as Deref>::Target {
		&self.0
	}
}

pub fn find(id: Id) -> Option<Codec<'static>> {
	unsafe {
		let ptr = avcodec_find_decoder(id.into());

		if ptr == ptr::null_mut() {
			None
		}
		else {
			Some(Codec::wrap(ptr))
		}
	}
}

pub fn find_by_name(name: &str) -> Option<Codec<'static>> {
	unsafe {
		let ptr = avcodec_find_decoder_by_name(CString::new(name).unwrap().as_ptr());

		if ptr == ptr::null_mut() {
			None
		}
		else {
			Some(Codec::wrap(ptr))
		}
	}
}