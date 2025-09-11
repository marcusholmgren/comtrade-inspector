<header class="mb-8">
    <h2 class="text-4xl font-bold tracking-tight">
        COMTRADE File Information
    </h2>
</header>
<div class="space-y-8">
    <section class="bg-[#181C21] rounded-lg p-6">
        <h3 class="text-xl font-semibold mb-4">
            File Information (from .CFG)
        </h3>
        <div
            class="grid grid-cols-1 md:grid-cols-2 gap-x-8 gap-y-4"
        >
            <div
                class="flex justify-between border-b border-[#3b4754] pb-2"
            >
                <p class="text-[#9dabb9]">Station</p>
                <p>Station A</p>
            </div>
            <div
                class="flex justify-between border-b border-[#3b4754] pb-2"
            >
                <p class="text-[#9dabb9]">Recorder ID</p>
                <p>REC001</p>
            </div>
            <div
                class="flex justify-between border-b border-[#3b4754] pb-2"
            >
                <p class="text-[#9dabb9]">Start Time</p>
                <p>15/01/2024, 09:59:58.123</p>
            </div>
            <div
                class="flex justify-between border-b border-[#3b4754] pb-2"
            >
                <p class="text-[#9dabb9]">Trigger Time</p>
                <p>15/01/2024, 10:00:00.000</p>
            </div>
            <div
                class="flex justify-between border-b border-[#3b4754] pb-2"
            >
                <p class="text-[#9dabb9]">File Type</p>
                <p>ASCII</p>
            </div>
            <div
                class="flex justify-between border-b border-[#3b4754] pb-2"
            >
                <p class="text-[#9dabb9]">Frequency</p>
                <p>50 Hz</p>
            </div>
        </div>
    </section>
    <section>
        <h3 class="text-xl font-semibold mb-4">
            Analog Channels
        </h3>
        <div class="overflow-x-auto bg-[#181C21] rounded-lg">
            <table class="min-w-full text-sm">
                <thead class="bg-[#283039] text-left">
                    <tr>
                        <th class="p-3">Channel</th>
                        <th class="p-3">Name</th>
                        <th class="p-3">Units</th>
                        <th class="p-3">Min</th>
                        <th class="p-3">Max</th>
                        <th class="p-3">Multiplier</th>
                        <th class="p-3">Offset</th>
                    </tr>
                </thead>
                <tbody>
                    <tr class="border-b border-[#3b4754]">
                        <td class="p-3 font-medium">1</td>
                        <td class="p-3 text-[#9dabb9]">
                            Voltage A
                        </td>
                        <td class="p-3 text-[#9dabb9]">V</td>
                        <td class="p-3 text-[#9dabb9]">
                            -1000
                        </td>
                        <td class="p-3 text-[#9dabb9]">1000</td>
                        <td class="p-3 text-[#9dabb9]">0.1</td>
                        <td class="p-3 text-[#9dabb9]">0</td>
                    </tr>
                    <tr class="border-b border-[#3b4754]">
                        <td class="p-3 font-medium">2</td>
                        <td class="p-3 text-[#9dabb9]">
                            Voltage B
                        </td>
                        <td class="p-3 text-[#9dabb9]">V</td>
                        <td class="p-3 text-[#9dabb9]">
                            -1000
                        </td>
                        <td class="p-3 text-[#9dabb9]">1000</td>
                        <td class="p-3 text-[#9dabb9]">0.1</td>
                        <td class="p-3 text-[#9dabb9]">0</td>
                    </tr>
                    <tr>
                        <td class="p-3 font-medium">3</td>
                        <td class="p-3 text-[#9dabb9]">
                            Voltage C
                        </td>
                        <td class="p-3 text-[#9dabb9]">V</td>
                        <td class="p-3 text-[#9dabb9]">
                            -1000
                        </td>
                        <td class="p-3 text-[#9dabb9]">1000</td>
                        <td class="p-3 text-[#9dabb9]">0.1</td>
                        <td class="p-3 text-[#9dabb9]">0</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </section>
    <section>
        <h3 class="text-xl font-semibold mb-4">
            Digital Channels
        </h3>
        <div class="overflow-x-auto bg-[#181C21] rounded-lg">
            <table class="min-w-full text-sm">
                <thead class="bg-[#283039] text-left">
                    <tr>
                        <th class="p-3 w-1/3">Channel</th>
                        <th class="p-3 w-1/3">Name</th>
                        <th class="p-3 w-1/3">Initial Value</th>
                    </tr>
                </thead>
                <tbody>
                    <tr class="border-b border-[#3b4754]">
                        <td class="p-3 font-medium">1</td>
                        <td class="p-3 text-[#9dabb9]">
                            Breaker Status
                        </td>
                        <td class="p-3 text-[#9dabb9]">0</td>
                    </tr>
                    <tr>
                        <td class="p-3 font-medium">2</td>
                        <td class="p-3 text-[#9dabb9]">
                            Trip Signal
                        </td>
                        <td class="p-3 text-[#9dabb9]">0</td>
                    </tr>
                </tbody>
            </table>
        </div>
    </section>
</div>
