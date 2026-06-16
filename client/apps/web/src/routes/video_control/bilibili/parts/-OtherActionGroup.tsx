import { Button } from "@workspace/ui/components/button"
import { History, Search, LoaderPinwheel } from "lucide-react"
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@workspace/ui/components/dialog"
import { Input } from "@workspace/ui/components/input"
import { Label } from "@workspace/ui/components/label"
import { useMqtt } from '@/components/mqtt/MqttContext'
import { useState } from "react"
export function OtherActionGroup() {
    const [searchValue, setSearchValue] = useState('')
    const mqttClient = useMqtt()
    const handleSearch = () => {
        mqttClient.publish('tv-web/log1997/receive', {
            action: 'bilibili',
            data: 'search',
            payload: searchValue
        })
    }
    const openHistory = () => {
        mqttClient.publish('tv-web/log1997/receive', {
            action: 'bilibili',
            data: 'history',
            payload: ''
        })
    }
    const openPostPage = () => {
        mqttClient.publish('tv-web/log1997/receive', {
            action: 'bilibili',
            data: 'post',
            payload: ''
        })
    }
    return <div className="other-action-group flex justify-between px-6 my-6">
        <div className="w-3/12 h-10">
            <Button className="w-full h-full" onClick={openHistory}>
                <History className="h-4 w-4" />
            </Button>
        </div>
        <div className="w-5/12 h-10">
            <Dialog>
                <DialogTrigger asChild>
                    <Button className="w-full h-full">
                        <Search className="h-4 w-4" />
                        Search
                    </Button>
                </DialogTrigger>
                <DialogContent className="sm:max-w-md">
                    <DialogHeader>
                        <DialogTitle>Search</DialogTitle>
                        <DialogDescription>
                            open search page .
                        </DialogDescription>
                    </DialogHeader>
                    <div className="flex items-center gap-2">
                        <div className="grid flex-1 gap-2">
                            <Label htmlFor="link" className="sr-only">
                                Link
                            </Label>
                            <Input
                                id="search-video"
                                type="text"
                                placeholder="Search..."
                                value={searchValue}
                                onChange={(e) => setSearchValue(e.target.value)}
                            />
                        </div>
                    </div>
                    <DialogFooter className="sm:justify-start">
                        <DialogClose asChild>
                            <Button type="button" onClick={handleSearch}>Submit</Button>
                        </DialogClose>
                    </DialogFooter>
                </DialogContent>
            </Dialog>
            {/* <Button className="w-full h-full">
                <Search className="h-4 w-4" />
                Search
            </Button> */}
        </div>
        <div className="w-3/12 h-10">
            <Button className="w-full h-full" onClick={openPostPage}>
                <LoaderPinwheel className="h-4 w-4" />
            </Button>
        </div>
    </div >
}
