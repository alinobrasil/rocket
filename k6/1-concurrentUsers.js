import http from 'k6/http';
import { sleep } from 'k6';

// tests for k6
// 1. concurrent requests for API itself: how many requests can it handle?
// 2. increasing block range: at what point will it break? --single user


export default function () {
    const params = {
        headers: {
            'x-api-key': 'abc',
        },
    };

    http.get('http://localhost:8000/fetch_data?block_start=18530000&block_end=18530100', params);
    sleep(1);
}
