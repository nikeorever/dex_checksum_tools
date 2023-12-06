use adler32::adler32;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

/// A `Dex` structure that holds the bytes of a DEX (Dalvik Executable) file.
///
/// # Fields
/// * `bytes` - A vector of bytes representing the contents of the DEX file.
///
/// # Examples
///
/// ```
/// use dex_checksum_tools::Dex;
///
/// if let Ok(mut dex) = Dex::try_from("/path/to/incorrect.dex") {
///     println!("Before Correcting: current_checksum {:?}", dex.current_checksum());
///     println!("Before Correcting: check_checksum {:?}", dex.check_checksum());
///     println!("Before Correcting: expect_checksum {:?}", dex.expect_checksum());
///     dex.correct_checksum();
///     println!("After Correcting: current_checksum {:?}", dex.current_checksum());
///     println!("After Correcting: check_checksum {:?}", dex.check_checksum());
///     println!("After Correcting: expect_checksum {:?}", dex.expect_checksum());
///     match dex.write_to_file("/path/to/correct.dex") {
///         Ok(_) => println!("Successfully wrote to file!"),
///         Err(e) => println!("Failed to write to file: {}", e),
///     }
/// }
/// ````
#[derive(Debug)]
pub struct Dex {
	bytes: Vec<u8>,
}

impl Dex {
	/// Calculates the current checksum from the DEX file's header.
	///
	/// This method extracts the checksum bytes that are stored at offset 8 through 11 in the DEX file header
	/// and converts them into a 4-byte array. The checksum is a part of the file's integrity verification
	/// and should match the expected checksum calculated over the rest of the file.
	///
	/// # Returns
	/// A 4-byte array representing the checksum stored in the DEX file header.
	///
	/// # Panics
	/// Panics if the slice of bytes cannot be converted into an array, which indicates an issue with the DEX file format.
	pub fn current_checksum(&self) -> [u8; 4] {
		self.bytes[8..12]
			.try_into()
			.expect("Could not convert slice to array!")
	}

	/// Calculates the expected checksum for the DEX file.
	///
	/// This method computes the Adler-32 checksum for the data part of the DEX file
	/// starting from byte 12 to the end of the file. The Adler-32 checksum is used
	/// for error-checking of the data. It should match the current checksum in the
	/// file header for the file to be considered valid.
	///
	/// # Returns
	/// A 4-byte array representing the expected checksum for the DEX file.
	///
	/// # Errors
	/// Returns an error if the Adler-32 checksum cannot be calculated.
	pub fn expect_checksum(&self) -> [u8; 4] {
		let mut hash = adler32(&self.bytes[12..]).expect("Unable to calculate adler32 checksum!");
		let mut buffer: [u8; 4] = [0; 4];
		for i in (0..4).rev() {
			buffer[i] = (hash % 256) as u8;
			hash >>= 8;
		}
		buffer.reverse();
		buffer
	}

	/// Checks if the current checksum matches the expected checksum.
	///
	/// This method compares the current checksum from the file's header
	/// with the expected checksum calculated over the data part of the file.
	///
	/// # Returns
	/// - `true` if the checksums match.
	/// - `false` otherwise.
	pub fn check_checksum(&self) -> bool {
		self.current_checksum() == self.expect_checksum()
	}

	/// Corrects the checksum in the DEX file header if it does not match the expected checksum.
	///
	/// This method calculates the expected checksum using the `expect_checksum` method
	/// and updates the bytes in the DEX file header if the current checksum is incorrect.
	/// After calling this method, the checksum in the file header should match the
	/// expected checksum for the data part of the DEX file.
	///
	/// # Returns
	/// - `true` if the checksum was uncorrected.
	/// - `false` otherwise.
	pub fn correct_checksum(&mut self) -> bool {
		let expect = self.expect_checksum();
		if self.current_checksum() != expect {
			self.bytes[8..12].copy_from_slice(&expect);
			true
		} else {
			false
		}
	}

	/// Writes the DEX file's bytes to the specified path.
	///
	/// This function creates a new file at the given `path` and writes the
	/// bytes representing the DEX file to it. If the file already exists,
	/// it will be truncated before writing.
	///
	/// # Arguments
	///
	/// * `path` - A string slice that holds the path where the file will be written.
	///
	/// # Returns
	///
	/// An `io::Result<()>` which is `Ok` if the file was written successfully,
	/// or an `Err` with more information if the file could not be written.
	pub fn write_to_file(&self, path: &str) -> io::Result<()> {
		File::create(path)?.write_all(&self.bytes)
	}
}

impl Display for Dex {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Dex {{ bytes: {:?} }}", self.bytes)
	}
}

impl TryFrom<File> for Dex {
	type Error = io::Error;

	fn try_from(mut f: File) -> Result<Self, Self::Error> {
		let mut bytes = Vec::<u8>::new();
		f.read_to_end(&mut bytes).map(|_| Dex { bytes })
	}
}

impl TryFrom<String> for Dex {
	type Error = io::Error;

	fn try_from(path: String) -> Result<Self, Self::Error> {
		match File::open(path) {
			Ok(f) => Dex::try_from(f),
			Err(e) => Err(e),
		}
	}
}

impl TryFrom<&str> for Dex {
	type Error = io::Error;

	fn try_from(path: &str) -> Result<Self, Self::Error> {
		Dex::try_from(String::from(path))
	}
}
