extern crate s3;

use std::{str, time};
use std::ops::Range;

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::region::Region;
use tokio::runtime::Runtime;
use s3::S3Error;

struct Storage {
    name: String,
    region: Region,
    credentials: Credentials,
    bucket: String,
    location_supported: bool,
}

const MESSAGE: &str = "I want to go to S3";

pub fn main() -> Result<(), S3Error> {
    /*
    "qiniu": {
        "key" : "vTdjr4n7zlLnnMjfP3VwmtixN94QIao20D-dpXda",
        "secret": "V9wmsvRpgutV8tuNnqDcFD9kilHIKACL8C9wH-4h",
        "url": "http://s3-qos.kodopoc.cn:9091",
        "bucket": "sc-test",
        "region_name": "cn-east-1",
        "chunksize": 66060288
    },
    */

    let credentials = Credentials::new(
        Some("vTdjr4n7zlLnnMjfP3VwmtixN94QIao20D-dpXda"),
        Some("V9wmsvRpgutV8tuNnqDcFD9kilHIKACL8C9wH-4h"),
        None, None, None)?;
    let region = Region::Custom {
        region: "cn-east-1".to_string(),
        endpoint: "http://s3-qos.kodopoc.cn:9091".to_string(),
    };

    let aws = Storage {
        name: "qiniu".into(),
        region: region,
        credentials: credentials,
        bucket: "filecoin-bucket-f07824-data".to_string(),
        location_supported: true,
    };

    /*
        "sealed/s-t07824-367", http://s3-qos.kodopoc.cn:9091/filecoin-bucket-f07824-data
        [3052000608..3052000640, 3052000512..3052000544, 3052000544..3052000576, 3052000576..3052000608, 3052000640..3052000672, 3052000672..3052000704, 3052000704..3052000736, 3052000736..3052000768]
        RESP Range: 3052000608-3052000640 / 0-32
        RESP Data: [79, 9b, b1, 9d, d2, 85, 18, de, 48, 3a, 10, 64, 7f, 1e, f, d7, d5, b9, bb, 92, 77, b9, cf, a, d2, a1, a5, 81, 87, 6d, 35, 1e, cc]
        RESP Range: 3052000512-3052000544 / 32-64
        RESP Data: [89, 6e, f8, da, 5a, 4c, ba, 4e, 69, ec, 4e, 3e, 85, 22, 3a, b5, d5, 55, ee, 3e, 31, 2e, a1, d2, 68, 7e, 9f, 1e, bf, 62, ce, 13, 85]
        RESP Range: 3052000544-3052000576 / 64-96
        RESP Data: [85, 5e, 33, e, bd, 2a, bc, 36, 97, af, a2, 25, 94, 2, 77, b3, 92, 28, 7b, 29, 4f, bf, 7e, 7a, 35, b9, dd, 1f, fa, 1d, b9, c, 8f]
        RESP Range: 3052000576-3052000608 / 96-128
        RESP Data: [8f, 38, c7, 10, c1, 49, ca, bd, 2e, 33, 88, 76, d6, 1a, 18, 7c, 4, d3, 2b, d9, ef, f5, e6, e8, 4e, 98, 9e, c, 8d, 1d, 89, b, 79]
        RESP Range: 3052000640-3052000672 / 128-160
        RESP Data: [cc, 1f, 29, b8, 83, 53, 13, 6b, 92, d, 52, 69, 28, 25, 69, 71, bf, b6, 85, 6a, d7, ab, df, 36, 1d, ee, 1e, 4f, 21, 3b, 54, 3, 2d]
        RESP Range: 3052000672-3052000704 / 160-192
        RESP Data: [2d, 12, fe, a8, 97, 64, 88, bc, 2, cd, 5f, 54, cf, a1, 77, f6, 26, dd, d2, 9, f9, c, bf, 2b, d0, 76, ec, 84, 67, ed, ba, 25, f6]
        RESP Range: 3052000704-3052000736 / 192-224
        RESP Data: [f6, 9c, fa, 3a, be, 11, a8, 10, 98, 32, 49, 43, cc, 4c, b5, d2, 7d, 7, 65, 98, d4, fd, 3d, 45, 5d, 65, 4, 54, 36, 5f, 22, d, d0]
        RESP Range: 3052000736-3052000768 / 224-256
        RESP Data: [d0, 5f, 9b, d1, b7, 71, 13, 18, 2f, 42, 70, b9, 6d, 43, 3a, 30, 10, 99, ab, bf, 68, 66, 5, 50, 46, 28, 6c, 42, a3, a4, 16, a, 9b]
     */

    let mut rt = Runtime::new()?;

    for backend in vec![aws] {
        println!("Running {}", backend.name);
        // Create Bucket in REGION for BUCKET
        let bucket = Bucket::new_with_path_style(&backend.bucket, backend.region,time::Duration::from_secs(5), backend.credentials)?;

        // [3052000608..3052000640, 3052000512..3052000544, 3052000544..3052000576, 3052000576..3052000608, 3052000640..3052000672, 3052000672..3052000704, 3052000704..3052000736, 3052000736..3052000768]
        let mut ranges = Vec::new();
        ranges.push(Range { start: 3052000608, end: 3052000640 });
        ranges.push(Range { start: 3052000512, end: 3052000544 });
        ranges.push(Range { start: 3052000544, end: 3052000576 });
        ranges.push(Range { start: 3052000576, end: 3052000608 });
        ranges.push(Range { start: 3052000640, end: 3052000672 });
        ranges.push(Range { start: 3052000672, end: 3052000704 });
        ranges.push(Range { start: 3052000704, end: 3052000736 });
        ranges.push(Range { start: 3052000736, end: 3052000768 });
        let (datas, code) = rt.block_on(bucket.get_object_multi_ranges("sealed/s-t07824-367", ranges)).unwrap();
        assert_eq!(206, code);

        for resp in datas {
            println!("{:?} / {:x?}", resp.range, resp.data);
            // assert_eq!(&MESSAGE[3..7], string);
        }
    }

    Ok(())
}
