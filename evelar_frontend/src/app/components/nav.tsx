'use client';

import React, { useState,  } from "react";
import Link from "next/link";
import { Menu, X } from "lucide-react";
import Button from "./connect_button";

export default function NavBar() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <nav className="absolute top-0 left-0 right-0 flex items-center justify-between px-8 py-6 bg-transparent">
      <h1 className="text-xl font-semibold text-cyan-400">Evelar</h1>
      
      {/* Mobile Menu Button */}
      <button className="md:hidden" onClick={() => setIsOpen(!isOpen)}>
        {isOpen ? <X className="text-white" size={24} /> : <Menu className="text-white" size={24} />}
      </button>
      
      {/* Navbar Links */}
      <div className={`absolute md:static top-16 left-0 right-0 md:flex md:space-x-6 text-gray-300 bg-[#0A0A12] md:bg-transparent md:p-0 p-4 ${isOpen ? "block" : "hidden"}`}>
        <a href="" className="block md:inline-block hover:text-white py-2">Home</a>
        <Link href={"createvent"} className="block px-4 py-2 hover:bg-gray-100">Create Event</Link>
        <div className="relative group">
          <button className="block md:inline-block hover:text-white flex items-center py-2">Explore <span className="ml-1">▼</span></button>
          <div className="absolute left-0 mt-2 w-40 bg-[#07091E] text-white rounded-lg shadow-lg hidden group-hover:block">
            <Link href={"dashboard"} className="block px-4 py-2 hover:bg-gray-100">Tech Event</Link>
            <Link href={"dashboard"} className="block px-4 py-2 hover:bg-gray-100">Create Event</Link>
            <a href="" className="block px-4 py-2 hover:bg-gray-100">DappOverAppEvent</a>
          </div>
        </div>
        <Link href={"eventdashboard"} className="block md:inline-block hover:text-white py-2">Dashboard</Link>
      </div>
      
          {/* Connect Wallet Button */}
          <Button btnText={"Connect Wallet"} />
      {/* <button className="hidden md:block bg-cyan-400 text-black px-4 py-2 rounded-lg">Connect Wallet</button> */}
    </nav>
  );
}
