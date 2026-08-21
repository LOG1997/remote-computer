import { createFileRoute } from '@tanstack/react-router'
import { WsConfigForm } from './parts/-WsConfig'
import { useWsConfig } from '@/stores'

export const Route = createFileRoute('/ws_config/')({
    component: function MqttConfigPage() {
        const configData = useWsConfig((state) => state.wsConfig)
        const setConfig = useWsConfig((state) => state.setConfig)
        // const getMqttConfig = async () => {

        //     if (configData) {
        //         setMqttConfig(configData)
        //     }
        // }
        const handleSubmitWsConfig = (data: any) => {
            console.log('handle submit ws', data);
            setConfig(data)
        }





        return (
            <div className="flex flex-col gap-4 w-full h-full items-center justify-center">
                <WsConfigForm handleSubmit={handleSubmitWsConfig} value={configData} />
            </div>
        )
    },
})

