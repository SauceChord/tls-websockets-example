# How I generated the key and crt file

I followed this tutorial: https://helpcenter.gsx.com/hc/en-us/articles/115015960428-How-to-Generate-a-Self-Signed-Certificate-and-Private-Key-using-OpenSSL

    > openssl req -x509 -sha256 -nodes -days 365 -newkey rsa:2048 -keyout localhost.key -out localhost.crt   
    Generating a RSA private key
    .........................................................................................+++++
    .+++++
    writing new private key to 'localhost.key'
    -----
    You are about to be asked to enter information that will be incorporated
    into your certificate request.
    What you are about to enter is what is called a Distinguished Name or a DN.
    There are quite a few fields but you can leave some blank
    For some fields there will be a default value,
    If you enter '.', the field will be left blank.
    -----
    Country Name (2 letter code) [AU]:SE
    State or Province Name (full name) [Some-State]:
    Locality Name (eg, city) []:
    Organization Name (eg, company) [Internet Widgits Pty Ltd]:
    Organizational Unit Name (eg, section) []:
    Common Name (e.g. server FQDN or YOUR name) []:localhost
    Email Address []:

Then I checked the certificate since I am unfamiliar with openssl according to https://stackoverflow.com/questions/1722181/how-to-determine-certificate-type-from-file

    > openssl x509 -in localhost.crt -text
    Certificate:
        Data:
            Version: 3 (0x2)
            Serial Number:
                46:20:36:f4:34:1e:c0:17:e3:0f:e0:26:2b:09:48:21:d2:e0:86:11
            Signature Algorithm: sha256WithRSAEncryption
            Issuer: C = SE, ST = Some-State, O = Internet Widgits Pty Ltd, CN = localhost
            Validity
                Not Before: Jul 28 23:49:38 2021 GMT
                Not After : Jul 28 23:49:38 2022 GMT
            Subject: C = SE, ST = Some-State, O = Internet Widgits Pty Ltd, CN = localhost
            Subject Public Key Info:
                Public Key Algorithm: rsaEncryption
                    RSA Public-Key: (2048 bit)
                    Modulus:
                        00:97:aa:17:cc:0c:39:59:67:26:9c:a4:23:75:0a:
                        8b:cc:5b:40:fb:7e:20:79:9c:51:0c:73:fa:ed:57:
                        04:32:10:e7:f6:50:e3:27:94:37:02:72:12:13:6b:
                        53:94:4c:aa:9e:66:58:1a:ca:02:af:83:a9:ac:b2:
                        d6:d1:9f:7a:36:32:22:4d:fc:ae:e6:f9:44:fc:9f:
                        02:57:3e:9f:66:c2:8a:87:cf:17:48:bd:d9:cb:2f:
                        99:48:69:e5:cb:97:d2:15:16:1c:6a:fa:13:68:87:
                        ae:3c:e8:37:3e:ae:8c:ea:96:d8:d2:16:b6:80:b1:
                        0e:61:38:13:c7:d6:76:db:cb:30:22:ab:be:35:ec:
                        81:36:37:96:78:50:ed:e5:2e:76:05:ef:16:bc:6c:
                        68:c8:b9:42:8a:c4:77:c1:94:d2:16:b8:c8:fa:47:
                        bc:c5:7a:34:5c:96:2b:00:6f:cb:9a:f8:ca:fa:e1:
                        eb:be:ae:47:53:f3:92:8f:68:e3:63:16:5f:01:28:
                        c5:46:5c:16:d5:bb:24:71:a3:09:da:2c:d6:94:34:
                        51:0e:e3:1b:b3:31:de:62:db:f2:48:a1:b0:a1:66:
                        3d:aa:b1:bd:68:9f:44:1a:01:7b:1f:8d:57:e9:68:
                        7c:56:aa:33:f7:21:9d:75:25:26:ea:0b:a9:54:5b:
                        91:15
                    Exponent: 65537 (0x10001)
            X509v3 extensions:
                X509v3 Subject Key Identifier:
                    EF:F8:FE:C8:64:C6:C4:97:EB:5D:37:42:BC:E4:B4:CB:2F:1E:86:8F
                X509v3 Authority Key Identifier:
                    keyid:EF:F8:FE:C8:64:C6:C4:97:EB:5D:37:42:BC:E4:B4:CB:2F:1E:86:8F

                X509v3 Basic Constraints: critical
                    CA:TRUE
        Signature Algorithm: sha256WithRSAEncryption
            65:8a:2a:b1:a1:01:d4:e9:02:e2:fa:38:27:d7:ae:cc:b9:5a:
            21:06:9e:7c:ae:79:92:4c:86:64:8e:16:f3:6f:24:6b:61:39:
            6b:28:e6:15:f3:2b:da:e6:00:d7:f3:2b:5d:f2:5b:1a:b4:6f:
            18:f4:61:39:1d:ec:67:0d:3f:55:2b:3a:c8:69:67:ef:f9:82:
            34:dd:41:a4:d3:b7:8c:06:6a:17:e3:c4:8c:5d:d9:f9:92:b0:
            01:20:56:86:1f:1b:30:3a:99:41:a4:08:b9:57:10:55:a4:a0:
            f4:2a:d6:9d:df:7d:36:2f:76:e5:89:c8:19:1d:a4:f1:22:46:
            a7:4a:5e:95:94:01:3c:27:f7:4b:a6:52:09:27:d8:9b:4b:ae:
            e1:27:16:a0:64:90:64:99:a6:42:c2:14:a5:fe:92:56:48:41:
            f1:91:ee:92:e3:61:0e:28:8b:e6:8d:86:68:80:61:05:a9:26:
            d9:82:26:b4:ec:aa:8f:3f:8b:3c:72:c3:d5:4b:05:8e:86:19:
            be:96:cd:ac:f4:9c:4b:53:51:80:6a:b9:76:7b:9e:d5:65:ba:
            85:04:35:c2:ae:6c:d3:77:9d:4a:66:da:0f:c8:42:26:cf:d5:
            55:1c:84:3e:91:92:21:0b:8e:4b:03:15:59:b8:9e:71:84:31:
            99:c2:74:78
    -----BEGIN CERTIFICATE-----
    MIIDkzCCAnugAwIBAgIURiA29DQewBfjD+AmKwlIIdLghhEwDQYJKoZIhvcNAQEL
    BQAwWTELMAkGA1UEBhMCU0UxEzARBgNVBAgMClNvbWUtU3RhdGUxITAfBgNVBAoM
    GEludGVybmV0IFdpZGdpdHMgUHR5IEx0ZDESMBAGA1UEAwwJbG9jYWxob3N0MB4X
    DTIxMDcyODIzNDkzOFoXDTIyMDcyODIzNDkzOFowWTELMAkGA1UEBhMCU0UxEzAR
    BgNVBAgMClNvbWUtU3RhdGUxITAfBgNVBAoMGEludGVybmV0IFdpZGdpdHMgUHR5
    IEx0ZDESMBAGA1UEAwwJbG9jYWxob3N0MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8A
    MIIBCgKCAQEAl6oXzAw5WWcmnKQjdQqLzFtA+34geZxRDHP67VcEMhDn9lDjJ5Q3
    AnISE2tTlEyqnmZYGsoCr4OprLLW0Z96NjIiTfyu5vlE/J8CVz6fZsKKh88XSL3Z
    yy+ZSGnly5fSFRYcavoTaIeuPOg3Pq6M6pbY0ha2gLEOYTgTx9Z228swIqu+NeyB
    NjeWeFDt5S52Be8WvGxoyLlCisR3wZTSFrjI+ke8xXo0XJYrAG/LmvjK+uHrvq5H
    U/OSj2jjYxZfASjFRlwW1bskcaMJ2izWlDRRDuMbszHeYtvySKGwoWY9qrG9aJ9E
    GgF7H41X6Wh8Vqoz9yGddSUm6gupVFuRFQIDAQABo1MwUTAdBgNVHQ4EFgQU7/j+
    yGTGxJfrXTdCvOS0yy8eho8wHwYDVR0jBBgwFoAU7/j+yGTGxJfrXTdCvOS0yy8e
    ho8wDwYDVR0TAQH/BAUwAwEB/zANBgkqhkiG9w0BAQsFAAOCAQEAZYoqsaEB1OkC
    4vo4J9euzLlaIQaefK55kkyGZI4W828ka2E5ayjmFfMr2uYA1/MrXfJbGrRvGPRh
    OR3sZw0/VSs6yGln7/mCNN1BpNO3jAZqF+PEjF3Z+ZKwASBWhh8bMDqZQaQIuVcQ
    VaSg9CrWnd99Ni925YnIGR2k8SJGp0pelZQBPCf3S6ZSCSfYm0uu4ScWoGSQZJmm
    QsIUpf6SVkhB8ZHukuNhDiiL5o2GaIBhBakm2YImtOyqjz+LPHLD1UsFjoYZvpbN
    rPScS1NRgGq5dnue1WW6hQQ1wq5s03edSmbaD8hCJs/VVRyEPpGSIQuOSwMVWbie
    cYQxmcJ0eA==
    -----END CERTIFICATE-----