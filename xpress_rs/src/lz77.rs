//! Plain LZ77 (MS-XCA) compression and decompression.
//!
//! ```
//! use xpress_rs::lz77::{LZ77Compressor, LZ77Decompressor};
//! use std::error::Error;
//!
//! let data_to_compress: Vec<u8> = b"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.".to_vec();
//! let compressor = LZ77Compressor::new(data_to_compress.clone());
//! let result: Result<Vec<u8>, Box<dyn Error>> = compressor.compress();
//! match result {
//!     Ok(compressed_data) => {
//!         let decompressor = LZ77Decompressor::new(compressed_data);
//!         let result: Result<Vec<u8>, Box<dyn Error>> = decompressor.decompress();
//!         match result {
//!             Ok(decompressed_data) => assert_eq!(decompressed_data, data_to_compress),
//!             Err(e) => panic!("Error: {e}"),
//!         }
//!     }
//!     Err(e) => panic!("Error: {e}"),
//! }
//! ```

use std::error::Error;
use std::fmt;

/// Compresses one Plain LZ77 buffer.
pub struct LZ77Compressor {
    data: Vec<u8>,
}

/// Decompresses one Plain LZ77 buffer.
pub struct LZ77Decompressor {
    data: Vec<u8>,
}

#[derive(Debug)]
struct Lz77Failure {
    message: &'static str,
}

impl LZ77Compressor {
    /// Stores `data` for a later [`compress`](Self::compress) call.
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Compresses the stored buffer with MS-XCA Plain LZ77.
    pub fn compress(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        lzxpress::data::compress(&self.data).map_err(failure)
    }
}

impl LZ77Decompressor {
    /// Stores a compressed buffer for a later [`decompress`](Self::decompress) call.
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    /// Decompresses the stored buffer with MS-XCA Plain LZ77.
    pub fn decompress(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        lzxpress::data::decompress(&self.data).map_err(failure)
    }
}

impl fmt::Display for Lz77Failure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message)
    }
}

impl Error for Lz77Failure {}

fn failure(error: lzxpress::error::Error) -> Box<dyn Error> {
    let message = match error {
        lzxpress::error::Error::MemLimit => "memory limit would be violated",
        lzxpress::error::Error::CorruptedData => "corrupt data",
        lzxpress::error::Error::Other => "xpress lz77 failed",
    };
    Box::new(Lz77Failure { message })
}
