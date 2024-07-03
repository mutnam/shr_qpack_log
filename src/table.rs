//编码表
//(字段名,字段值,(0,客户端，1服务端,2两者))
pub const FIELD_TABLE: [(usize, &'static str, &'static str,u8); 99] = [
    (0, ":authority", "",0),
    (1, ":path", "/",0),
    (2, "age", "0",1),//主要是缓存服务器的响应,此缓存存在了多少秒
    (3, "content-disposition", "",1),//指定文件浏览器储存方式attachment,下载，attachment;filename="example.pdf"//https://www.rfc-editor.org/rfc/rfc6266.html
    (4, "content-length", "0",2),//http3.可以不需要，机制不一样，分块编码（chunked encoding)不能携带
    (5, "cookie", "",0),//https://www.rfc-editor.org/rfc/rfc6265.html
    (6, "date", "",1),//生成内容的时间,https://www.rfc-editor.org/rfc/rfc9110.html
    (7, "etag", "",1),//资源版本，资源匹配返回304，精度高
    (8, "if-modified-since", "",0),//验证资源last-modified
    (9, "if-none-match", "",0),//客户端资源版本，文件分块上传，缓存
    (10, "last-modified", "",1),//资源最后修改时间用于缓存，同一秒内修改无法识别
    (11, "link", "",1),//资源关系，好处多多
    (12, "location", "",1),//重定向,201,301重定向
    (13, "referer", "",0),//从链接从哪里来
    (14, "set-cookie", "",1),
    (15, "method", "CONNECT",0),
    (16, "method", "DELETE",0),
    (17, "method", "GET",0),
    (18, "method", "HEAD",0),
    (19, "method", "OPTIONS",0),
    (20, "method", "POST",0),
    (21, "method", "PUT",0),
    (22, "scheme", "http",0),
    (23, "scheme", "https",0),
    (24, "status", "103",0),
    (25, "status", "200",0),
    (26, "status", "304",0),
    (27, ":status", "404",0),
    (28, ":status", "503",0),
    (29, "accept", "*/*",0),//客户端愿意接受的文档类型
    (30, "accept", "application/dns-message",0),
    (31, "accept-encoding", "gzip, deflate, br",0),//客户端愿意接受编码方式，415拒绝编码
    (32, "accept-ranges", "bytes" ,1),//范围请求Range: bytes=1024000-，从多少字节开始，回应客户端的range，大文件断点下载
    (33, "access-control-allow-headers", "cache-control",1),//跨域允许字段
    (34, "access-control-allow-headers", "content-type",1),//跨域允许字段
    (35, "access-control-allow-origin", "*",1),//跨域指定哪些资源可以访问api，https://example.com
    (36, "cache-control", "max-age=0",0),//客户端希望得到的缓存策略不缓存，等
    (37, "cache-control", "max-age=2592000",0),
    (38, "cache-control", "max-age=604800",0),
    (39, "cache-control", "no-cache",0),//
    (40, "cache-control", "no-store",0),
    (41, "cache-control", "public, max-age=31536000",0),
    (42, "content-encoding", "br",1),//回应accept-encoding//不设置不编码
    (43, "content-encoding", "gzip",1),
    (44, "content-type", "application/dns-message",1),//accept响应，或406找不到类型
    (45, "content-type", "application/javascript",1),
    (46, "content-type", "application/json",1),
    (47, "content-type", "application/x-www-form-urlencoded",1),
    (48, "content-type", "image/gif",1),
    (49, "content-type", "image/jpeg",1),
    (50, "content-type", "image/png",1),
    (51, "content-type", "text/css",1),
    (52, "content-type", "text/html; charset=utf-8",1),
    (53, "content-type", "text/plain",1),
    (54, "content-type", "text/plain;charset=utf-8",1),
    (55, "range", "bytes=0-",0),//请求范围Range: bytes=1024000-1536000，不满足服务端返回406
    (56, "strict-transport-security", "max-age=31536000",1),//在特定时间内使用https,会将添加大浏览器HSTS列表中
    (57, "strict-transport-security", "max-age=31536000; includesubdomains",1),
    (58, "strict-transport-security", "max-age=31536000; includesubdomains; preload",1),
    (59, "vary", "accept-encoding",1),//内容优化，不同浏览器显示不同内容
    (60, "vary", "origin",1),
    (61, "x-content-type-options", "nosniff",1),//不要对Content-Type进行检测
    (62, "x-xss-protection", "1; mode=block",1),//预防xss攻击
    (63, ":status", "100",1),
    (64, ":status", "204",1),
    (65, ":status", "206",1),
    (66, ":status", "302",1),
    (67, ":status", "400",1),
    (68, ":status", "403",1),
    (69, ":status", "421",1),
    (70, ":status", "425",1),
    (71, ":status", "500",1),
    (72, "accept-language", "",0),//语言环境
    (73, "access-control-allow-credentials", "FALSE",1),
    (74, "access-control-allow-credentials", "TRUE",1),
    (75, "access-control-allow-headers", "*",1),
    (76, "access-control-allow-methods", "get",1),
    (77, "access-control-allow-methods", "get, post, options",1),
    (78, "access-control-allow-methods", "options",1),
    (79, "access-control-expose-headers", "content-length",1),
    (80, "access-control-request-headers", "content-type",1),
    (81, "access-control-request-method", "get",1),
    (82, "access-control-request-method", "post",1),
    (83, "alt-svc", "clear",1),//替代服务器，可以从h2 替换为h3，平滑过度
    (84, "authorization", "",1),//身份认证
    (85, "content-security-policy", "script-src 'none'; object-src 'none'; base-uri 'none'",1),//指定源加载防止xss攻击
    (86, "early-data", "1",0),//接收0-RTT早期数据
    (87, "expect-ct", "",1),//SCTs
    (88, "forwarded", "",0),//代理链,服务端直接发回数据不经过代理
    (89, "if-range", "",0),//与range可判断是否过期
    (90, "origin", "",1),//跨域指定哪些资源可以访问
    (91, "purpose", "prefetch",0),//预取资源
    (92, "server", "",1),//服务器产品php,rust
    (93, "timing-allow-origin", "*",1),//指示哪些源允许查看原本因跨源限制而被置零的时间属性值，从哪个域名来的请求通过
    (94, "upgrade-insecure-requests", "1",0),//301升级https也可以不管
    (95, "user-agent", "",0),//客户端软件名称，识别设备
    (96, "x-forwarded-for", "",0),//记录真实ip,负载均衡
    (97, "x-frame-options", "deny",1),//<frame>, <iframe>, <embed> 或 <object>内容是否能在里面显示
    (98, "x-frame-options", "sameorigin",1),
];