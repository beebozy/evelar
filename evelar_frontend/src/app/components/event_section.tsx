import React from 'react'

export const EventSection = () => {
    const events = [
        { title: "Web3 Events", description: "Our digital platform offers services ranging from digitizing AI and man.", count: 78 },
        { title: "AI Events", description: "Our digital platform offers services ranging from digitizing AI and man.", count: 78 },
        { title: "Entertainment Events", description: "Our digital platform offers services ranging from digitizing AI and man.", count: 78 },
    ];
    return (
        <section className="py-16 px-8 bg-[#07091E] text-white">
            <div className="overflow-x-auto flex space-x-4 ">
                {events.map((event, index) => (
                    <div key={index} className="min-w-[300px]  p-6 rounded-lg shadow-md border border-gray-700 py-16 px-8">
                        <div className='flex justify-between '>
                            <h3 className="text-lg font-semibold">{event.title}</h3>
                            <button className=" px-6 py-2  bg-transparent border border-gray-700 border-1 rounded-3xl text-xs">{event.count} Events</button>
                        </div>


                        <p className="text-gray-400 mt-2">{event.description}</p>
                        <button className="mt-4 px-4 py-2 bg-transparent border border-gray-700 border-1 rounded-3xl">Explore</button>
                    </div>))}
            </div>
        </section>
    );
}


