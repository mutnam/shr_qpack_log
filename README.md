## h3 qpack 测试工具

```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_worksa() {
       let v=[1, 64, 130, 0, 0, 209, 215, 80, 143, 53, 43, 250, 241, 235, 192, 235, 173, 114, 30, 155, 141, 62, 251, 255, 193, 95, 80, 231, 208, 127, 102, 162, 129, 176, 218, 224, 82, 26, 235, 160, 188, 139, 30, 99, 37, 134, 217, 117, 118, 92, 83, 250, 205, 143, 126, 140, 255, 74, 80, 110, 165, 83, 17, 73, 212, 255, 106, 16, 244, 214, 52, 154, 58, 11, 246, 167, 43, 199, 144, 186, 74, 150, 4, 184, 62, 212, 255, 115, 165, 53, 162, 227, 12, 78, 148, 214, 202, 254, 8, 121, 10, 189, 69, 75, 31, 218, 151, 167, 176, 244, 149, 128, 133, 197, 192, 184, 95, 101, 229, 221, 113, 77, 195, 148, 118, 25, 134, 217, 117, 118, 92, 221, 223];
       
        let x = Decoder::new(v.to_vec()).parse();
        println!("###{:#?}",x);
    }
}
```

## 标准文档
[h3 HEADERS 帧 https://www.rfc-editor.org/rfc/rfc9114#frame-headers](https://www.rfc-editor.org/rfc/rfc9114#frame-headers)
[QPACK](https://www.rfc-editor.org/rfc/rfc9204.html)


## 输出结果

```
 ###Ok(
   HeaderFrame {
        frame_type: Some(
            1,
        ),
        bytes: Some(
            [1, 64, 130, 0, 0, 209, 215, 80, 143, 53, 43, 250, 241, 235, 192, 235, 173, 114, 30, 155, 141, 62, 251, 255, 193, 95, 80, 231, 208, 127, 102, 162, 129, 176, 218, 224, 82, 26, 235, 160, 188, 139, 30, 99, 37, 134, 217, 117, 118, 92, 83, 250, 205, 143, 126, 140, 255, 74, 80, 110, 165, 83, 17, 73, 212, 255, 106, 16, 244, 214, 52, 154, 58, 11, 246, 167, 43, 199, 144, 186, 74, 150, 4, 184, 62, 212, 255, 115, 165, 53, 162, 227, 12, 78, 148, 214, 202, 254, 8, 121, 10, 189, 69, 75, 31, 218, 151, 167, 176, 244, 149, 128, 133, 197, 192, 184, 95, 101, 229, 221, 113, 77, 195, 148, 118, 25, 134, 217, 117, 118, 92, 221, 223],
        ),
        length: Some(
            130,
        ),
        field_section: Some(
            FieldSection {
                required_insert_count: Some(
                    0,
                ),
                bytes: Some(
                    [209, 215, 80, 143, 53, 43, 250, 241, 235, 192, 235, 173, 114, 30, 155, 141, 62, 251, 255, 193, 95, 80, 231, 208, 127, 102, 162, 129, 176, 218, 224, 82, 26, 235, 160, 188, 139, 30, 99, 37, 134, 217, 117, 118, 92, 83, 250, 205, 143, 126, 140, 255, 74, 80, 110, 165, 83, 17, 73, 212, 255, 106, 16, 244, 214, 52, 154, 58, 11, 246, 167, 43, 199, 144, 186, 74, 150, 4, 184, 62, 212, 255, 115, 165, 53, 162, 227, 12, 78, 148, 214, 202, 254, 8, 121, 10, 189, 69, 75, 31, 218, 151, 167, 176, 244, 149, 128, 133, 197, 192, 184, 95, 101, 229, 221, 113, 77, 195, 148, 118, 25, 134, 217, 117, 118, 92, 221, 223],
                ),
                sign: Some(
                    0,
                ),
                delta_base: Some(
                    0,
                ),
                field_lines: [
                    IndexedFieldLine {
                        prefix: "1",
                        t: 1,
                        index: 17,
                        value: "method=GET",
                    },
                    IndexedFieldLine {
                        prefix: "1",
                        t: 1,
                        index: 23,
                        value: "scheme=https",
                    },
                    LiteralFieldLinewithNameReference {
                        prefix: "01",
                        n: 0,
                        t: 1,
                        name_index: 0,
                        name_value: ":authority",
                        h: 1,
                        value_length: 15,
                        value_string: "im.ywywapp.com:4999",
                    },
                    IndexedFieldLine {
                        prefix: "1",
                        t: 1,
                        index: 1,
                        value: ":path=/",
                    },
                    LiteralFieldLinewithNameReference {
                        prefix: "01",
                        n: 0,
                        t: 1,
                        name_index: 95,
                        name_value: "user-agent",
                        h: 1,
                        value_length: 103,
                        value_string: "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; WCodeNet/2.0; +https://http3.wcode.net/) Chrome/116.0.1938.76 Safari/537.36",
                    },
                    IndexedFieldLine {
                        prefix: "1",
                        t: 1,
                        index: 29,
                        value: "accept=*/*",
                    },
                    IndexedFieldLine {
                        prefix: "1",
                        t: 1,
                        index: 31,
                        value: "accept-encoding=gzip, deflate, br",
                    },
                ],
            },
        ),
    },
)
```