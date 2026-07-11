import { Slider } from "@workspace/ui/components/slider"
import { useState, useEffect, useRef, useMemo } from "react"
import { useMqtt } from "@/components/mqtt/MqttContext"
import { Volume2, VolumeOff } from "lucide-react"
import { debounce } from 'lodash-es';
export function VolumeAction() {
    const { onMessage, subscribe, publish } = useMqtt()
    const [volumeValue, setVolumeValue] = useState([0])
    const [volumeMute, setVolumeMute] = useState(false)
    // ref 用于标记当前是否正在由用户拖动滑块
    const isInteractingRef = useRef(false);
    useEffect(() => {
        subscribe('tv-web/volume/state/send')
        publish('tv-web/volume/state/receive', "0")
        const unsubscribe = onMessage((msg) => {
            // 这里会立即执行，不依赖 React 渲染周期

            if (msg.topic === 'tv-web/volume/state/send') {
                if (!isInteractingRef.current) {
                    const data = JSON.parse(msg.payload);
                    console.log('⚡ 设置音量', data);

                    const volume = data.volume;
                    const mute = data.mute; // 如果后续需要用到 mute 也可以保留
                    setVolumeMute(mute);
                    setVolumeValue([volume]);
                }
            }
        });

        // 组件卸载或依赖变化时，取消注册，防止内存泄漏
        return () => {
            unsubscribe();
        };
    }, [])
    // 在组件外部或内部创建防抖函数
    // mute发送switch才是切换，其他的不执行操作
    const debouncedPublish = useMemo(() => {
        return debounce((publishFn: (topic: string, message: any) => void, val: number, mute: string) => {
            publishFn('tv-web/control/volume', JSON.stringify({ mute, volume: val }));
        }, 200);
    }, []);
    useEffect(() => {
        return () => {
            debouncedPublish.cancel();
        };
    }, [debouncedPublish]);
    const onChangeVolume = (value: number[]) => {
        setVolumeValue(value);
        debouncedPublish(publish, value[0], "none");
    }
    const onChangeVolumeStart = () => {
        navigator.vibrate?.(50);
        isInteractingRef.current = true;
    };

    const onChangeVolumeEnd = () => {
        isInteractingRef.current = false;
        debouncedPublish.flush();
    };
    const onChangeMute = () => {
        navigator.vibrate?.(50);
        setVolumeMute(!volumeMute);
        debouncedPublish(publish, volumeValue[0], 'switch');
    }
    return (
        <div className="flex w-full max-w-sm gap-4 justify-center items-center" >
            <div className="w-4/5 flex justify-between gap-6 items-center">

                <Slider
                    className="h-30 min-h-30 [&_[data-slot='slider-track']]:h-12 [&_[data-slot='slider-thumb']]:hidden"
                    value={volumeValue}
                    onValueChange={onChangeVolume}
                    onValueCommit={onChangeVolumeEnd} // 当用户停止拖动时触发
                    onPointerDown={onChangeVolumeStart} // 当用户按下鼠标/触摸时触发
                    min={0}
                    max={100}
                    step={1}
                />
                <div className="w-10 h-10 cursor-pointer" onClick={onChangeMute}>
                    {
                        volumeMute ? <VolumeOff className="h-10 w-10" /> : <Volume2 className="h-10 w-10" />
                    }
                </div>
            </div>
        </div >
    )
}
