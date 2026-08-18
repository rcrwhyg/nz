# 后置：审计与暴力破解

实现前必须单独获得用户批准。默认不编译进发布二进制（日后用 feature flag，默认关）。

仅用于本机、实验室或书面授权目标。禁止对未授权系统编写利用过程或公网演示测试。

| 号 | 标题 | 原树节点（约） |
|----|------|----------------|
| 73 | Simulate presence (arp and ping) | netaudit / alive |
| 74 | Flood random fragments | netaudit IP |
| 75 | Switch CAM flood | netaudit ETH |
| 76 | Synflood | netaudit TCP |
| 77 | Predictable seqnum check | netaudit TCP |
| 78 | Reset every TCP packet | netaudit TCP |
| 79 | Acknowledge every TCP SYN | netaudit TCP |
| 80 | Periodically send ARP replies | netaudit ARP |
| 81 | Send ICMP4 timestamp | netaudit ICMP |
| 82 | Sniff + ICMP dest unreachable | netaudit ICMP |
| 83 | Sniff + ICMP time exceeded | netaudit ICMP |
| 84 | Sniff + ICMP parameter problem | netaudit ICMP |
| 85 | Sniff + ICMP source quench | netaudit ICMP |
| 86 | Sniff + ICMP redirect | netaudit ICMP |
| 98 | Flood syslog messages | syslog + flood |
| 101 | Brute force telnet | bruteforce |
| 130 | Brute force ftp | bruteforce |
| 131 | Brute force http (site) | bruteforce |
| 132 | Brute force http (proxy) | bruteforce |

共 20 个。未批准前：`spec/` 可有草稿，`src/` 不得有实现。
