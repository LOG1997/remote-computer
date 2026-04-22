import { AmdIcon, IntelIcon } from "@/components/icons"
import { Cpu } from 'lucide-react'

interface Props {
    cpu?: string
}
export default function DeviceIcon(props: Props) {
    const { cpu = "default" } = props
    return (
        <>
            {
                cpu.includes('Amd') ? <AmdIcon /> :
                    cpu.includes('Intel') ? <IntelIcon /> :
                        <Cpu />
            } {/* 默认图标 */}
        </>
    )
}
