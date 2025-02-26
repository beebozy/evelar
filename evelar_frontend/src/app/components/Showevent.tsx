'use client'
import React, { useState } from 'react';
import { LogOut, User, Wallet, Gift, LayoutDashboard, MapPin } from 'lucide-react';

// Define the types
type NavItem = 'dashboard' | 'wallet' | 'rewards' | 'profile';

interface Event {
  id: number;
  title: string;
  description: string;
  date: string;
  location: string;
  attendees: number;
}

export default function ShowEvent() {
  const [activeNav, setActiveNav] = useState<NavItem>('dashboard');
  const [events] = useState<Event[]>([
    {
      id: 1,
      title: "Evento De Mascot",
      description: "Our digital platform offers services ranging from digitizing AI and man...",
      date: "10:00am, 15th March 2025",
      location: "Lekki Tech Zone Park",
      attendees: 1052,
    },
    {
      id: 2,
      title: "Evento De Mascot",
      description: "Our digital platform offers services ranging from digitizing AI and man...",
      date: "10:00am, 15th March 2025",
      location: "Lekki Tech Zone Park",
      attendees: 1052,
    },
    {
      id: 3,
      title: "Evento De Mascot",
      description: "Our digital platform offers services ranging from digitizing AI and man...",
      date: "10:00am, 15th March 2025",
      location: "Lekki Tech Zone Park",
      attendees: 1052,
    },
  ]);

  return (
    <div className="min-h-screen bg-[#07091e] text-white">
      <Sidebar activeNav={activeNav} setActiveNav={setActiveNav} />
      <main className="pl-[240px]">
        <Header />
        <div className="p-6">
          <div className="mb-8 grid grid-cols-3 gap-6">
            <div className="rounded-lg bg-white/5 p-6">
              <h3 className="mb-4 text-sm text-gray-400">Total Events</h3>
              <p className="text-4xl font-bold">17</p>
            </div>
            <div className="rounded-lg bg-white/5 p-6">
              <h3 className="mb-4 text-sm text-gray-400">Total Registrations</h3>
              <p className="text-4xl font-bold">21403</p>
            </div>
            <div className="flex items-center justify-between rounded-lg bg-white/5 p-6">
              <div>
                <h3 className="mb-4 text-sm text-gray-400">Ongoing Events</h3>
                <div className="space-y-1">
                  <p className="flex items-center gap-2">
                    <span className="text-[#00e2ff]">◆</span> 3 Events
                  </p>
                  <p className="flex items-center gap-2">
                    <span className="text-[#00e2ff]">◆</span> 5749 Registrants
                  </p>
                </div>
              </div>
              <button className="bg-[#00e2ff] text-black hover:bg-[#00e2ff]/90 px-4 py-2 rounded-lg">
                Add Event
              </button>
            </div>
          </div>

          {events.map((event) => (
            <EventCard key={event.id} event={event} />
          ))}
        </div>
      </main>
      <Footer />
    </div>
  );
}

interface SidebarProps {
  activeNav: NavItem;
  setActiveNav: React.Dispatch<React.SetStateAction<NavItem>>;
}

function Sidebar({ activeNav, setActiveNav }: SidebarProps) {
  const navItems = [
    { name: "dashboard", icon: LayoutDashboard, label: "Dashboard" },
    { name: "wallet", icon: Wallet, label: "Wallet" },
    { name: "rewards", icon: Gift, label: "Rewards" },
    { name: "profile", icon: User, label: "Profile" },
  ];

  return (
    <aside className="fixed left-0 top-0 h-full w-[240px] border-r border-white/10 p-6">
      <div className="flex h-full flex-col">
        <a href="/" className="mb-8 text-xl font-semibold">
          <span className="text-[#00e2ff]">E</span>velar
        </a>

        <nav className="flex flex-1 flex-col justify-evenly py-12">
          {navItems.map((item) => (
            <a
              key={item.name}
              href={`/${item.name}`}
              className={`flex items-center gap-3 rounded-lg px-4 py-3 ${
                activeNav === item.name ? "bg-[#00e2ff] text-black" : "text-gray-400 hover:bg-white/5"
              }`}
              onClick={(e) => {
                e.preventDefault();
                setActiveNav(item.name as NavItem);
              }}
            >
              <item.icon size={20} />
              {item.label}
            </a>
          ))}
        </nav>

        <div className="mt-auto">
          <button className="flex w-full items-center gap-3 text-gray-400 hover:text-white px-4 py-2 rounded-lg">
            <LogOut size={20} />
            Log Out
          </button>
        </div>
      </div>
    </aside>
  );
}

function Header() {
  return (
    <header className="flex items-center justify-between p-6">
      <input
        placeholder="Search....."
        className="w-[600px] rounded-lg border-white/10 bg-white/5 px-4 py-2 text-white"
      />
      <button className="flex items-center gap-2 px-4 py-2 rounded-lg hover:bg-white/5">
        <div className="h-8 w-8 rounded-full bg-gray-600 flex items-center justify-center">
          AM
        </div>
        <span>Alex Moribo</span>
      </button>
    </header>
  );
}

interface EventCardProps {
  event: Event;
}

function EventCard({ event }: EventCardProps) {
  return (
    <div className="mb-6 rounded-lg bg-white/5 p-6">
      <div className="mb-4 flex items-start justify-between">
        <div>
          <h3 className="mb-2 text-xl font-semibold">{event.title}</h3>
          <p className="mb-4 text-sm text-gray-400">{event.description}</p>
          <div className="flex items-center gap-6 text-sm text-gray-400">
            <div className="flex items-center gap-2">
              <span className="text-[#00e2ff]">◆</span>
              {event.date}
            </div>
            <div className="flex items-center gap-2">
              <MapPin size={16} className="text-[#00e2ff]" />
              {event.location}
            </div>
          </div>
        </div>
      </div>
      <button className="border border-[#00e2ff] text-[#00e2ff] px-4 py-2 rounded-lg hover:bg-[#00e2ff]/10">
        View Event
      </button>
    </div>
  );
}

function Footer() {
  return (
    <footer className="fixed bottom-0 left-0 w-full border-t border-white/10 bg-[#07091e] p-6 pl-[240px]">
      <p className="text-sm text-gray-400">2025. All Rights Reserved</p>
    </footer>
  );
}
