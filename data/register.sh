#!/bin/bash
curl -X PUT -d '{"Datacenter": "dc1", "Node": "google", "Address": "www.google.com", "Service": {"Service": "gsearch", "Port": 80, "Tags": ["release"]}}' http://127.0.0.1:8500/v1/catalog/register
