import { ChevronLeft, ChevronRight, ChevronUp, ChevronDown } from 'lucide-react'
import { useWebSocket } from "@/components/ws/WebsocketProvider"
import { useSwipeable } from 'react-swipeable';

export function DirectionPart() {
     const { sendMessage, subscribe, readyState } = useWebSocket()
    const handleMove = (direction: 'up' | 'down' | 'left' | 'right') => {
        navigator.vibrate(50)
        sendMessage({
                   topic: "BrowserControl",
                   token: "1231212",
                   date_time: new Date().getTime(),
                   command: {
                       command_type: "douyin",
                       param:direction,
                         payload:""
                   },
               })
    }
  const handleEnter = () => {
    console.log("双击")
      sendMessage({
                 topic: "BrowserControl",
                 token: "1231212",
                 date_time: new Date().getTime(),
                 command: {
                   command_type: "douyin",
                    param:"enter",
                      payload:""
                 },
             })
    }


    const handlers = useSwipeable({
       onSwiped: (eventData) => console.log('滑动了', eventData),
      onSwipedRight: () => { console.log('向右滑了'),handleMove('right') }, // 有各种方向专用的事件[reference:7]
      onSwipedLeft: () => { console.log('作画');handleMove('left') },
      onSwipedDown: () => { console.log('下滑');handleMove('down') },
      onSwipedUp: () => { console.log('上滑');handleMove('up') },
       onSwiping: (eventData) => console.log('正在滑动', eventData.deltaX, eventData.deltaY),
       trackMouse: true, // 也允许用鼠标模拟滑动[reference:8]
       delta: 100, // 触发滑动事件的最小距离（像素）[reference:9]
     });
    return (<div className='w-full flex justify-center'>
      <div {...handlers} onDoubleClick={handleEnter} style={{ touchAction: 'none' }} className='w-full h-120 bg-yellow-300'></div>
    </div>)
}
