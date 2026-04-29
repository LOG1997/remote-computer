import { createFileRoute } from '@tanstack/react-router'
import { useMqtt } from '@/components/mqtt/MqttContext'
import { Button } from '@workspace/ui/components/button';
import { Dot } from 'lucide-react'
import type { RemoteControlType } from '@/routes/remote-control/-type'
import { RemoteControlTypeList } from '@/routes/remote-control/-type'

export const Route = createFileRoute('/remote-control/bilibili/')({
    component: RouteComponent,
})

function RouteComponent() {
    const { subscribe, publish, messages, isConnected } = useMqtt();
    const subscribeTopic = () => {
        subscribe('tv/remote-control/computer/bilibili');
    }
    const publishTopic = (operation: RemoteControlType) => {
        publish('tv/remote-control/computer/bilibili', {
            type: operation,
            time: new Date().getTime()
        });
    }

    // useEffect(() => {
    //     subscribeTopic();
    // }, [])
    return <div>
        <h2>连接状态：{
            isConnected ?
                <div className="flex gap-1 items-center">
                    <Dot className="text-green-500" strokeWidth={8} />
                    <span>已连接</span>
                </div> :
                <div className="flex gap-1 items-center">
                    <Dot className="text-red-500" strokeWidth={8} />
                    <span>未连接</span>
                </div>
        }
        </h2>

        <Button onClick={subscribeTopic}>订阅</Button>
        <Button onClick={() => publishTopic(RemoteControlTypeList.UP)}>上</Button>

        {messages.map((m, i) => (
            <div key={i}>[{m.time}] {m.topic}: {m.payload}</div>
        ))}
    </div>
}
