import React from 'react'

export const Bottomnav = () => {
    return (
        <div>
            <hr className="h-px bg-gray-200 border-0 dark:bg-gray-700" />
         <nav className="flex items-center justify-between px-8 py-6 bg-transparent mt-10">
            <h1 className="text-xl font-semibold text-cyan-400">Evelar</h1>

            {/* Navbar Links */}
            <div className={`absolute md:static top-16 left-0 right-0 md:flex md:space-x-6 text-gray-300 bg-[#0A0A12] md:bg-transparent md:p-0 p-4 `}>
                <a href="#" className="block md:inline-block hover:text-white py-2">About Evelar</a>
                <a href="#" className="block md:inline-block hover:text-white py-2">Create Event</a>
                <div className="relative group">
                    <button className="block md:inline-block hover:text-white flex items-center py-2">Explore <span className="ml-1">▼</span></button>
                    <div className="absolute left-0 mt-2 w-40 bg-white text-black rounded-lg shadow-lg hidden group-hover:block">
                        <a href="#" className="block px-4 py-2 hover:bg-gray-100">Option 1</a>
                        <a href="#" className="block px-4 py-2 hover:bg-gray-100">Option 2</a>
                    </div>
                </div>
                <a href="#" className="block md:inline-block hover:text-white py-2">News and Media</a>
            </div>

   
            <p className="block md:inline-block hover:text-white py-2 text-white"> 2025 All Right Reserved </p>
         </nav>
        </div>
    );
}

