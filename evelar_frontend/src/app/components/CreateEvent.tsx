'use client'
import { useState } from "react";

export default function CreateEventPage() {
  // State for form inputs
  const [eventData, setEventData] = useState({
    eventName: "",
    location: "",
    time: "",
    date: "",
    phoneNumber: "",
    numberOfTickets: "",
  });

  // Handle input changes
  const handleChange = (e: any) => {
    const { name, value } = e.target;
    setEventData({ ...eventData, [name]: value });
  };

  // Handle form submission
  const handleSubmit = (e: any) => {
    e.preventDefault();
    console.log("Event Data:", eventData); // Process event creation here
  };

  return (
    <div className="min-h-screen bg-[#07091e] text-white relative overflow-hidden">
      <main className="relative z-10 py-16 px-4 sm:px-6 lg:px-8">
        <div className="max-w-7xl mx-auto">
          <h1 className="text-4xl md:text-5xl font-bold text-center mb-4">
            Create New Event
          </h1>
          <p className="text-center text-gray-400 max-w-3xl mx-auto mb-16">
            Organize and share your event with the community. Fill in the details below to get started.
          </p>

          <div className="grid md:grid-cols-2 gap-8 max-w-6xl mx-auto">
            {/* Create Event Form */}
            <div className="space-y-6">
              <h2 className="text-2xl font-semibold mb-8">Event Details</h2>
              <form onSubmit={handleSubmit} className="space-y-4">
                <input
                  type="text"
                  name="eventName"
                  placeholder="Event Name"
                  value={eventData.eventName}
                  onChange={handleChange}
                  className="w-full p-2 rounded-md bg-transparent border border-gray-700"
                />
                <input
                  type="text"
                  name="location"
                  placeholder="Location"
                  value={eventData.location}
                  onChange={handleChange}
                  className="w-full p-2 rounded-md bg-transparent border border-gray-700"
                />
                <input
                  type="time"
                  name="time"
                  placeholder="Time"
                  value={eventData.time}
                  onChange={handleChange}
                  className="w-full p-2 rounded-md bg-transparent border border-gray-700"
                />
                <input
                  type="date"
                  name="date"
                  placeholder="Date"
                  value={eventData.date}
                  onChange={handleChange}
                  className="w-full p-2 rounded-md bg-transparent border border-gray-700"
                />
                <input
                  type="tel"
                  name="phoneNumber"
                  placeholder="Phone Number"
                  value={eventData.phoneNumber}
                  onChange={handleChange}
                  className="w-full p-2 rounded-md bg-transparent border border-gray-700"
                />
                <input
                  type="number"
                  name="numberOfTickets"
                  placeholder="Number of Tickets"
                  value={eventData.numberOfTickets}
                  onChange={handleChange}
                  className="w-full p-2 rounded-md bg-transparent border border-gray-700"
                />

                <button
                  type="submit"
                  className="w-full p-2 rounded-md bg-[#00e2ff] text-black hover:bg-[#6dc2fc]"
                >
                  Create Event
                </button>
              </form>
            </div>

            {/* Preview Card */}
            <div className="rounded-lg p-8 space-y-6">
              {/* Preview the event data here if needed */}
              <h2 className="text-2xl font-semibold">Event Preview</h2>
              <p><strong>Event Name:</strong> {eventData.eventName}</p>
              <p><strong>Location:</strong> {eventData.location}</p>
              <p><strong>Time:</strong> {eventData.time}</p>
              <p><strong>Date:</strong> {eventData.date}</p>
              <p><strong>Phone Number:</strong> {eventData.phoneNumber}</p>
              <p><strong>Number of Tickets:</strong> {eventData.numberOfTickets}</p>
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}
