import { Progress } from "@workspace/ui/components/progress"
import { Slider } from "@workspace/ui/components/slider"
import { useState, useEffect } from "react"
import { useMqtt } from "@/components/mqtt/MqttContext"

export function VolumeAction() {
    const mqtt_client = useMqtt()
    const [value, setValue] = useState([50])
    useEffect(() => {
        mqtt_client.subscribe('tv-web/log1997/send')
        mqtt_client.publish('tv-web/volume/state/receive', '{"type": "volume", "value": 50}')
        mqtt_client.messages.forEach((message) => {
            // if (message.topic === 'tv-web/log1997/send') {
            console.log('只爱斯大林卡拉', message.payload)
            // setValue([parseInt(message.payload)])
            // }
        })
    }, [])
    const onChangeVolume = (value: number[]) => {
        console.log('onChangeVolume', value)
        mqtt_client.publish('tv-web/volume/state/receive', '{"type": "volume", "value": ' + value[0] + '}')
    }
    return (
        <div className="flex w-full max-w-sm flex-col gap-4" >
            <Progress value={value[0]} />
            <Slider
                value={value}
                onValueChange={onChangeVolume}
                min={0}
                max={100}
                step={1}
            />
        </div >
    )
}
