import { Undo2, Redo2 } from "lucide-react";
import { Button } from '@workspace/ui/components/button'
import { useMqtt } from '@/components/mqtt/MqttContext'
// 快进 快退 全屏 
export function VideoOption() {
    const { publish } = useMqtt()
    const handleNavigate = (direction: 'back' | 'forward') => {
        publish('tv-web/log1997/receive', {
            action: 'navigate',
            data: direction
        })
    }
    return (
        <div className="other-action-group flex justify-between px-6 my-6">
            <div className="w-3/12 h-10">
                <Button className="w-full h-full" onClick={() => handleNavigate('back')}>
                    <Undo2 className="h-4 w-4" />
                </Button>
            </div>

            <div className="w-3/12 h-10">
                <Button className="w-full h-full" onClick={() => handleNavigate('forward')}>
                    <Redo2 className="h-4 w-4" />
                </Button>
            </div>
        </div>
    )
}
