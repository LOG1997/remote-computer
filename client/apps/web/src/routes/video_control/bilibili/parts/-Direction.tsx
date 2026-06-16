import { ChevronLeft, ChevronRight, ChevronUp, ChevronDown } from 'lucide-react'
import { useMqtt } from '@/components/mqtt/MqttContext'

export function DirectionPart() {
    const mqttClient = useMqtt()
    const handleMove = (direction: 'up' | 'down' | 'left' | 'right') => {
        mqttClient.publish('tv-web/log1997/receive', {
            action: 'bilibili',
            data: direction
        })
    }
    const handleEnter = () => {
        mqttClient.publish('tv-web/log1997/receive', {
            action: 'bilibili',
            data: 'enter'
        })
    }

    return (<div className='w-full flex justify-center'>

        <div className="relative w-2/3 aspect-square rounded-full bg-linear-to-br from-gray-700 to-gray-900 shadow-2xl shadow-black/50 flex items-center justify-center p-4">
            {/* 使用 grid 3x3 布局 */}
            <div className="grid grid-cols-3 grid-rows-3 gap-2 w-full h-full">
                {/* 上 */}
                <button
                    className="col-start-2 row-start-1 rounded-t-2xl rounded-b-lg bg-linear-to-b from-gray-600 to-gray-800 shadow-[0_6px_0_#111] active:shadow-[0_2px_0_#111] active:translate-y-1 transition-all duration-75 flex items-center justify-center text-white text-3xl font-bold"
                    onClick={() => handleMove('up')}
                    aria-label="上"
                >
                    <ChevronUp />
                </button>

                {/* 左 */}
                <button
                    className="col-start-1 row-start-2 rounded-l-2xl rounded-r-lg bg-linear-to-r from-gray-600 to-gray-800 shadow-[0_6px_0_#111] active:shadow-[0_2px_0_#111] active:translate-y-1 transition-all duration-75 flex items-center justify-center text-white text-3xl font-bold"
                    onClick={() => handleMove('left')}
                    aria-label="左"
                >
                    <ChevronLeft />
                </button>

                {/* 中间确定 */}
                <button
                    className="col-start-2 row-start-2 rounded-full bg-linear-to-tr from-amber-600 to-yellow-700 shadow-inner shadow-black/50 text-white text-2xl font-bold flex items-center justify-center active:scale-95 transition-all duration-75"
                    onClick={handleEnter}
                    aria-label="确定"
                >
                    OK
                </button>

                {/* 右 */}
                <button
                    className="col-start-3 row-start-2 rounded-r-2xl rounded-l-lg bg-linear-to-l from-gray-600 to-gray-800 shadow-[0_6px_0_#111] active:shadow-[0_2px_0_#111] active:translate-y-1 transition-all duration-75 flex items-center justify-center text-white text-3xl font-bold"
                    onClick={() => handleMove('right')}
                    aria-label="右"
                >
                    <ChevronRight />
                </button>

                {/* 下 */}
                <button
                    className="col-start-2 row-start-3 rounded-b-2xl rounded-t-lg bg-linear-to-t from-gray-600 to-gray-800 shadow-[0_6px_0_#111] active:shadow-[0_2px_0_#111] active:translate-y-1 transition-all duration-75 flex items-center justify-center text-white text-3xl font-bold"
                    onClick={() => handleMove('down')}
                    aria-label="下"
                >
                    <ChevronDown />
                </button>

                {/* 四个角落留空（占位但不显示） */}
                <div className="col-start-1 row-start-1" />
                <div className="col-start-3 row-start-1" />
                <div className="col-start-1 row-start-3" />
                <div className="col-start-3 row-start-3" />
            </div>
        </div>
    </div>)
}
