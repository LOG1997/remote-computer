import { createFileRoute } from '@tanstack/react-router'
import { MqttConfigForm } from './parts/-MqttConfig'
import { useMqttConfig } from '@/stores'

export const Route = createFileRoute('/mqtt_config/')({
    component: function MqttConfigPage() {
        const configData = useMqttConfig((state) => state.mqttConfig)
        const setConfig = useMqttConfig((state) => state.setConfig)
        // const getMqttConfig = async () => {

        //     if (configData) {
        //         setMqttConfig(configData)
        //     }
        // }
        const handleSubmitMqtt = (data: any) => {
            console.log('handle submit mqtt', data);
            setConfig(data)
        }





        return (
            <div className="flex flex-col gap-4 w-full h-full items-center justify-center">
                <MqttConfigForm handleSubmit={handleSubmitMqtt} value={configData} />
            </div>
        )
    },
})

