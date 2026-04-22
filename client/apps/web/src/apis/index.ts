/* eslint-disable @typescript-eslint/no-explicit-any */


export const sendShutDownCommand = async ({ data, config }: { data?: { key: string, immediate: boolean }, config: any }) => {
    const response = await fetch(config.baseUrl + '/shutdown', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(data),
    })
    console.log('resppp', response)
    // if (!response.ok) {
    //     throw new Error(`HTTP error! status: ${response.status}`);
    // }

    return response.json();
}

export const getDeviceStatus = async ({ config }: { config: any }) => {
    const response = await fetch(config.baseUrl + '/getStatus', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
    })
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }

    return response.json();
}

export const getDeviceInfo = async ({ config }: { config: any }) => {
    const response = await fetch(config.baseUrl + '/getDeviceInfo', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
    })
    if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
    }

    return response.json();
}