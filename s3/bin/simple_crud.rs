extern crate s3;

use std::str;
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
        bucket: "sc-test".to_string(),
        location_supported: true,
    };

    // let aws_public = Storage {
    //     name: "aws-public".into(),
    //     region: "eu-central-1".parse()?,
    //     credentials: Credentials::anonymous()?,
    //     bucket: "rust-s3-public".to_string(),
    //     location_supported: true,
    // };

    // let minio = Storage {
    //     name: "minio".into(),
    //     region: Region::Custom {
    //         region: "us-east-1".into(),
    //         endpoint: "https://minio.adder.black".into(),
    //     },
    //     credentials: Credentials::from_profile(Some("minio"))?,
    //     bucket: "rust-s3".to_string(),
    //     location_supported: false,
    // };

    // let yandex = Storage {
    //     name: "yandex".into(),
    //     region: "ru-central1".parse()?,
    //     credentials: Credentials::from_profile(Some("yandex"))?,
    //     bucket: "soundcloud".to_string(),
    //     location_supported: false,
    // };

    let mut rt = Runtime::new()?;

    for backend in vec![aws] {
        println!("Running {}", backend.name);
        // Create Bucket in REGION for BUCKET
        let bucket = Bucket::new_with_path_style(&backend.bucket, backend.region, backend.credentials)?;

        // List out contents of directory
        let results = rt.block_on(bucket.list("".to_string(), None)).unwrap();
        for list in results {
            println!("{:?}", list.contents.len());
        }

        // Make sure that our "test_file" doesn't exist, delete it if it does. Note
        // that the s3 library returns the HTTP code even if it indicates a failure
        // (i.e. 404) since we can't predict desired usage. For example, you may
        // expect a 404 to make sure a fi le doesn't exist.
        //    let (_, code) = bucket.delete("test_file")?;
        //    assert_eq!(204, code);

        // Put a "test_file" with the contents of MESSAGE at the root of the
        // bucket.
        let (_, code) = rt.block_on(bucket.put_object("test_file", MESSAGE.as_bytes())).unwrap();
        // println!("{}", bucket.presign_get("test_file", 604801)?);
        assert_eq!(200, code);

        // Get the "test_file" contents and make sure that the returned message
        // matches what we sent.
        let (data, code) = rt.block_on(bucket.get_object("test_file")).unwrap();
        let string = str::from_utf8(&data)?;
        // println!("{}", string);
        assert_eq!(200, code);
        assert_eq!(MESSAGE, string);

        let (data, code) = rt.block_on(bucket.get_object_range("test_file", 3, Some(6))).unwrap();
        let string = str::from_utf8(&data)?;
        // println!("{}", string);
        assert_eq!(206, code);
        assert_eq!(&MESSAGE[3..7], string);

        let mut ranges = Vec::new();
        ranges.push(Range { start: 2, end: 4 });
        ranges.push(Range { start: 5, end: 7 });
        let (datas, code) = rt.block_on(bucket.get_object_multi_ranges("test_file", ranges)).unwrap();
        assert_eq!(206, code);

        for resp in datas {
            let string = str::from_utf8(&resp.data)?;
            println!("{:?} / {}", resp.range, string);
            // assert_eq!(&MESSAGE[3..7], string);
        }

        if backend.location_supported {
            // Get bucket location
            println!("{:?}", rt.block_on(bucket.location()).unwrap());
        }

        rt.block_on(bucket.put_object_tagging("test_file", &[("test", "tag")])).unwrap();
        println!("Tags set");
        let (tags, _status) = rt.block_on(bucket.get_object_tagging("test_file")).unwrap();
        println!("{:?}", tags);

        // Test with random byte array

        let random_bytes: Vec<u8> = (0..3072).map(|_| 33).collect();
        let (_, code) = rt.block_on(bucket.put_object("random.bin", random_bytes.as_slice())).unwrap();
        assert_eq!(200, code);
        let (data, code) = rt.block_on(bucket.get_object("random.bin")).unwrap();
        assert_eq!(code, 200);
        assert_eq!(data.len(), 3072);
        assert_eq!(data, random_bytes);
    }

    Ok(())
}
