'use client';

import React, { useState } from 'react';
import Link from 'next/link';

// Type definitions
interface SidebarProps {
  activeNav: string;
  setActiveNav: (nav: string) => void;
}

interface Event {
  title: string;
  tag: string;
  date: string;
}

// Sidebar Component
const Sidebar: React.FC<SidebarProps> = ({ activeNav, setActiveNav }) => {
  const navItems = [
    { name: 'dashboard', label: 'Dashboard' },
    { name: 'wallet', label: 'Wallet' },
    { name: 'rewards', label: 'Rewards' },
    { name: 'profile', label: 'Profile' },
  ];

  return (
    <aside className="fixed left-0 top-0 h-full w-[240px] border-r border-white/10 p-6 bg-[#0a0c2d]">
      <div className="flex h-full flex-col">
        <Link href="/" className="mb-8 text-xl font-semibold text-white">
          <span className="text-[#00e2ff]">E</span>velar
        </Link>

        <nav className="flex flex-1 flex-col justify-evenly py-12">
          {navItems.map((item) => (
            <a
              key={item.name}
              href={`/${item.name}`}
              className={`flex items-center gap-3 rounded-lg px-4 py-3 cursor-pointer ${
                activeNav === item.name
                  ? 'bg-[#00e2ff] text-black'
                  : 'text-gray-400 hover:bg-white/5'
              }`}
              onClick={() => setActiveNav(item.name)}
            >
              {item.label}
            </a>
          ))}
        </nav>

        <div className="mt-auto">
          <button className="flex w-full items-center gap-3 text-gray-400 hover:text-white">
            <span className="material-icons">logout</span>
            Log Out
          </button>
        </div>
      </div>
    </aside>
  );
};

// Event List Component
const EventList: React.FC = () => {
  const events: Event[] = [
    { title: 'Tech Conference', tag: 'Technology', date: '2024-11-20' },
    { title: 'Art Workshop', tag: 'Creativity', date: '2024-12-05' },
    { title: 'Music Festival', tag: 'Entertainment', date: '2025-01-15' },
  ];

  return (
    <div className="ml-[260px] p-6">
      <h1 className="text-4xl md:text-5xl font-bold text-center mb-4">
        Evento De Mascot
      </h1>
      <p className="text-center text-gray-400 max-w-3xl mx-auto mb-8">
        Explore the latest events and happenings!
      </p>

      <h2 className="text-3xl text-center mb-4 text-[#00e2ff]">Everything goes ooo..</h2>

      {/* Statistics Section */}
      <div className="grid grid-cols-4 gap-4 mb-8">
        <div className="bg-[#1c1f3b] p-6 rounded-lg text-center">
          <h2 className="text-2xl font-bold">Total Events</h2>
          <p className="text-[#00e2ff] text-3xl">15</p>
        </div>
        <div className="bg-[#1c1f3b] p-6 rounded-lg text-center">
          <h2 className="text-2xl font-bold">Total Registrations</h2>
          <p className="text-[#00e2ff] text-3xl">250</p>
        </div>
        <div className="bg-[#1c1f3b] p-6 rounded-lg text-center">
          <h2 className="text-2xl font-bold">Ongoing Events</h2>
          <p className="text-[#00e2ff] text-3xl">3</p>
        </div>
        <div className="flex justify-center items-center">
          <button className="bg-[#00e2ff] text-black py-3 px-6 rounded-lg font-bold">
            Host Event
          </button>
        </div>
      </div>

      {/* Event Listings */}
      <div className="grid grid-cols-1 gap-6">
        {events.map((event, index) => (
          <div
            key={index}
            className="p-6 rounded-lg shadow-md bg-[#1c1f3b] text-white w-full"
          >
            <h2 className="text-2xl font-semibold">{event.title}</h2>
            <p className="text-gray-400">Tag: {event.tag}</p>
            <p className="text-gray-500">Date: {event.date}</p>
          </div>
        ))}
      </div>
    </div>
  );
};

// Main Component
const EventPage: React.FC = () => {
  const [activeNav, setActiveNav] = useState<string>('dashboard');

  return (
    <div className="min-h-screen bg-[#050A2F] text-white relative overflow-hidden">
      <Sidebar activeNav={activeNav} setActiveNav={setActiveNav} />
      <main className="relative z-10 py-16 px-4 sm:px-6 lg:px-8 ml-[260px]">
        <div className="max-w-7xl mx-auto">
          <EventList />
        </div>
      </main>
    </div>
  );
};

export default EventPage;
