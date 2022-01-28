## 调用Dnspod的api来做ddns

从环境变量中读取dnspod的token和域名

```shell
export dnspod_token="xxxxxx,594xxxxxxxxxxxxxxxxxxxx73"
export dnspod_domain="example.com"
export dnspod_subdomain="www"
## 查询ip的url，可以不指定
export dnspod_ip_url="https://xxxx.com/ip"
```