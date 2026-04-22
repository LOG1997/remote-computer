/* eslint-disable @typescript-eslint/no-explicit-any */
import request from './request'


export const sendShutDownCommand = ({ data, config }: { data?: any, config: any }) => {
    return request({
        baseUrl: config ? config.baseUrl : undefined,
        url: '/api/device/shutdown',
        method: 'post',
        data
    })
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