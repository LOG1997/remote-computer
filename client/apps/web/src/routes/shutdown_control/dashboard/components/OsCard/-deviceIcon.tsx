import { LinuxDevice, WindowsDevice, MacDevice, DefaultDevice } from "@/components/icons"


interface Props {
    device?: 'linux' | 'mac' | 'windows' | string
}
export default function DeviceIcon(props: Props) {
    const { device = "default" } = props
    return (
        <>
            {
                device.includes('Linux') ? <LinuxDevice /> :
                    device.includes('Mac') ? <MacDevice /> :
                        device.includes('Windows') ? <WindowsDevice /> :
                            <DefaultDevice />
            } {/* 默认图标 */}
        </>
    )
}
