// Import the EventPage component correctly
import EventPage from "@/app/components/Myform";
import ShowEventPage from "@/app/components/Showevent";

import CreateEvent  from "./components/CreateEvent";
export default function Home() {
  return (
    <div>
      {/* <EventPage /> */}
      {/* <CreateEvent/> */}
      <ShowEventPage/>
    </div>
  );
}
