# UDC - convert toml configuration to unbound configuration


## example

```bash
# genearte unbound configuration to `/etc/unbound` directory
udc -o /etc/unbound unbound.toml
```

unbound.toml:

```toml
[[cn]]
name = "default"
dns = [
    "2001:4860:4860::8888#dns.google",
    "2620:fe::9#dns.quad9.net",
    "2606:4700:4700::1111#cloudflare-dns.com",
    "8.8.8.8#dns.google",
    "9.9.9.9#dns.quad9.net",
    "1.1.1.1#cloudflare-dns.com",
]
domains = [
    ".",
]

[[cn]]
name = "domestic"
dns = [
    "1.12.12.12@853#dot.pub",
    "120.53.53.53@853#dot.pub",
    "223.5.5.5@853#dns.alidns.com",
    "223.6.6.6@853#dns.alidns.com",
    "2400:3200::1@853#dns.alidns.com",
    "2400:3200:baba::1@853#dns.alidns.com",
]
domains = [
    "cn",
    "baidu.com",
]
```

output: /etc/unbound/cn.conf

```conf
forward-zone:
    name: '.'
    forward-addr: 2001:4860:4860::8888#dns.google
    forward-addr: 2620:fe::9#dns.quad9.net
    forward-addr: 2606:4700:4700::1111#cloudflare-dns.com
    forward-addr: 8.8.8.8#dns.google
    forward-addr: 9.9.9.9#dns.quad9.net
    forward-addr: 1.1.1.1#cloudflare-dns.com
    forward-tls-upstream: yes

forward-zone:
    name: 'cn'
    forward-addr: 1.12.12.12@853#dot.pub
    forward-addr: 120.53.53.53@853#dot.pub
    forward-addr: 223.5.5.5@853#dns.alidns.com
    forward-addr: 223.6.6.6@853#dns.alidns.com
    forward-addr: 2400:3200::1@853#dns.alidns.com
    forward-addr: 2400:3200:baba::1@853#dns.alidns.com
    forward-tls-upstream: yes

forward-zone:
    name: 'baidu.com'
    forward-addr: 1.12.12.12@853#dot.pub
    forward-addr: 120.53.53.53@853#dot.pub
    forward-addr: 223.5.5.5@853#dns.alidns.com
    forward-addr: 223.6.6.6@853#dns.alidns.com
    forward-addr: 2400:3200::1@853#dns.alidns.com
    forward-addr: 2400:3200:baba::1@853#dns.alidns.com
    forward-tls-upstream: yes
```
