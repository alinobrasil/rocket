import http from 'k6/http';
import { sleep } from 'k6';

export default function () {
    let delay = __VU * 1000; // Delay in milliseconds, increases with each VU
    sleep(delay / 1000);

    const apiKeyHeaders = {
        headers: {
            'x-api-key': 'abc',
        },
    };

    // First API call
    let response = http.get('http://localhost:8000/fetch_data?block_start=18530000&block_end=18530500', apiKeyHeaders);
    // console.log("\n\n Response body********************: ", response.body)
    let task_id = String(response.body)


    // Continuously check the status of the task
    while (true) {
        let checkResponse;
        try {
            checkResponse = http.get(`http://localhost:8000/check_data?task_id=${task_id}`, apiKeyHeaders);
            let response_json = JSON.parse(checkResponse.body)
            let status = response_json.status
            // console.log(`Task: ${task_id}  |  Status: ${status}`);



            // Break the loop if the task is completed or failed
            if (status === 'Completed' || status === 'Failed') {
                break;
            }
        } catch (error) {
            console.log("***************************ERROR*************************")
            console.log("Error:", error)
            console.log("Response body:", checkResponse.body)
        }

        // Pause before checking again
        sleep(1);
    }

    // sleep(1)
}
