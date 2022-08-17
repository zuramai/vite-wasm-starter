import './style.css'
import init, { use_counter } from "../crates/hello/pkg"


const run = async () => {
  await init()
  
  const counterBtn = use_counter()
  const card = document.querySelector('.card')
  
  card?.appendChild(counterBtn)
}

run()