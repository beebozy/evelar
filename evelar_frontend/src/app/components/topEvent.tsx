export const TopEvent = () => {
    const events = [
        { title: "Web3 Events", description: "Our digital platform offers services ranging from digitizing AI and man.", count: 78 },
        { title: "AI Events", description: "Our digital platform offers services ranging from digitizing AI and man.", count: 78 },
        { title: "Entertainment Events", description: "Our digital platform offers services ranging from digitizing AI and man.", count: 78 },
        { title: "Web3 Events", description: "Our digital platform offers services ranging from digitizing AI and man.", count: 78 },
        { title: "AI Events", description: "Our digital platform offers services ranging from digitizing AI and man.", count: 78 },
        { title: "Entertainment Events", description: "Our digital platform offers services ranging from digitizing AI and man.", count: 78 },
    ];

    return (
        <section className="py-16 px-8 pb-20 bg-[#07091E] text-white">
            <div className="text-start mb-8 text-xl font-semibold">Top Events</div>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {events.map((event, index) => (
                    <div 
                        key={index} 
                        className="bg-[#0E1129] p-6 rounded-lg shadow-md border border-gray-700"
                    >
                        <div className="flex justify-between items-center">
                            <h3 className="text-lg font-semibold">{event.title}</h3>
                            <button className="px-6 py-2 bg-transparent border border-gray-700 rounded-3xl text-xs">
                                {event.count} Events
                            </button>
                        </div>
                        <p className="text-gray-400 mt-2">{event.description}</p>
                        <button className="mt-4 px-4 py-2 bg-transparent border border-gray-700 rounded-3xl">
                            Explore
                        </button>
                    </div>
                ))}
            </div>
        </section>
    );
};
