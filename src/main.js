import { mount } from 'svelte';
import App from './App.svelte';
import './assets/css/main.css';
import './assets/fonts/roboto.css';

const app = mount(App,{
  target: document.body,
});

export default app;
