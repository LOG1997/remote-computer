
import { ChevronLeft } from "lucide-react"
import { useNavigate } from '@tanstack/react-router'

export function LeftHeader() {
    const navigate = useNavigate()
    const gotoRoute = () => {
        navigate({ to: '/ws_config' })
    }
    return <div className="shrink-0 flex items-center" onClick={gotoRoute}>
        <ChevronLeft className="h-6 w-6 text-gray-600" />
        <span className='text-xs'>去配置</span>
    </div>
}
