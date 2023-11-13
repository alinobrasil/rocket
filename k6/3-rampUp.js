import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
    stages: [
        { duration: '10s', target: 2 },
        { duration: '10s', target: 4 },
        { duration: '10s', target: 6 },
        { duration: '10s', target: 8 },
        { duration: '10s', target: 10 },
        { duration: '10s', target: 12 },
        { duration: '10s', target: 14 },
        { duration: '10s', target: 16 },
        { duration: '10s', target: 18 },
        { duration: '10s', target: 20 },

    ],
};

export default function () {
    const params = {
        headers: {
            'x-api-key': 'abc',
        },
    };

    const res = http.get('http://localhost:8000/fetch_data?block_start=18530000&block_end=18530100', params);
    check(res, { 'status was 200': (r) => r.status == 200 });
    sleep(1);
}
