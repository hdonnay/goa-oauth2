# goa-oauth2
[![Crates.io](https://img.shields.io/crates/v/goa-oauth2)](https://crates.io/crates/goa-oauth2)

This utility retrieves OAuth2 tokens from the GNOME Online Accounts service.
It's extremely useful for applications like msmtp:

```.msmtprc
defaults
auth oauthbearer

account gmail
host smtp.gmail.com
from example@gmail.com
user example@gmail.com
passwordeval goa-oauth2 example@gmail.com
```
