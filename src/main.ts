import './style.css'
import init, { use_counter } from "hello"

await init()

const counterBtn = use_counter()
const card = document.querySelector('.card')

card?.appendChild(counterBtn)

