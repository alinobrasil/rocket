import http from 'k6/http';
import { sleep } from 'k6';

export default function () {
    const params = {
        headers: {
            'x-api-key': 'abc',
        },
    };

    http.get('http://localhost:8000/fetch_data?block_start=18530000&block_end=18531000', params);
    sleep(1);
}
