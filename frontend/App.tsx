import React, {useEffect} from "react";
import "./App.css";
import {get_actor} from "./backend_connect/ic_agent";
import NavBar from "./components/spesific/nav_bar";
import Pages from "./pages/main";
import {BrowserRouter} from "react-router-dom";
import {NavAppBar} from "./components/spesific/app_bar";
import SearchPopper from "./components/genral/search_popper";

function App() {
    const [message, setMessage] = React.useState("");
    const [islogin, setLogin] = React.useState(false);

    async function doGreet() {
        setMessage("calling the backend....");
        let actor = await get_actor();
        const greeting = await actor.greet("world");
        setMessage(greeting);
    }


    useEffect(() => {
        doGreet();
    }, []);

    console.log("world  == connect is correct", message);
    return (
        <BrowserRouter>

            <div>
                <NavAppBar/>
                <NavBar>
                    <SearchPopper/>
                    <Pages/>

                </NavBar>
            </div>
        </BrowserRouter>
    )
        ;
}

export default App;
