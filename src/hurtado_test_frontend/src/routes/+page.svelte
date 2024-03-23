<script>
  import "../index.css";
  import { backend } from "$lib/canisters";
  import { authenticate, get_files, logout, upload_file, is_authenticated as is_auth, formatBytes } from "$lib/sdk";
  import {onMount} from "svelte"

  let greeting = "";

  async function fetchFiles() {
    files = await get_files();
  }

  let files = []
  let is_authenticated = false

  onMount(() => {
    let task = async () => {
      try {
      files = await get_files();
      console.log(files)

      } catch(err) {
        console.log(err)
      }
      
      is_authenticated = await is_auth()
    }

    task().then(d => {

    })
    
  })



  
  function onselect(event) {
    const files = event.target.files;
    for (let index = 0; index < files.length; index++) {
      const element = files[index];
       upload_file(element).catch(err => {
        console.log(err)
      }).then(e => {
        console.log(e)
        fetchFiles().then()
      })

      
    }
    
    console.log(files)
  }
</script>

<main>
  <!--
  This example requires some changes to your config:
  
  ```
  // tailwind.config.js
  module.exports = {
    // ...
    plugins: [
      // ...
      require('@tailwindcss/forms'),
    ],
  }
  ```
-->
<!--
  When the mobile menu is open, add `overflow-hidden` to the `body` element to prevent double scrollbars

  Open: "fixed inset-0 z-40 overflow-y-auto", Closed: ""
-->
<header>
  <nav class="bg-white border-gray-200 px-4 lg:px-6 py-2.5 dark:bg-gray-800">
      <div class="flex flex-wrap justify-between items-center mx-auto max-w-screen-xl">
          <a href="https://flowbite.com" class="flex items-center">
              <img src="https://flowbite.com/docs/images/logo.svg" class="mr-3 h-6 sm:h-9" alt="Flowbite Logo" />
              <span class="self-center text-xl font-semibold whitespace-nowrap dark:text-white">Flowbite</span>
          </a>
          {#if is_authenticated}
          <div class="flex items-center lg:order-2">
            <button on:click={e => {
             logout().finally(() => {
              is_authenticated = true
              files = []
             })
            }} class="inline-flex items-center rounded-md bg-red-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-red-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-red-600">Disconnect</button>
            <!-- <a href="#" class="text-white bg-primary-700 hover:bg-primary-800 focus:ring-4 focus:ring-primary-300 font-medium rounded-lg text-sm px-4 lg:px-5 py-2 lg:py-2.5 mr-2 dark:bg-primary-600 dark:hover:bg-primary-700 focus:outline-none dark:focus:ring-primary-800">Get started</a> -->
           
        </div>
          {:else}
          <div class="flex items-center lg:order-2">
            <button on:click={e => {
              authenticate().then(() => {
                is_authenticated = true
              }).catch(err => {
                console.log(err)
              })
            }} class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">SignIn with Internet Identity</button>
            <!-- <a href="#" class="text-white bg-primary-700 hover:bg-primary-800 focus:ring-4 focus:ring-primary-300 font-medium rounded-lg text-sm px-4 lg:px-5 py-2 lg:py-2.5 mr-2 dark:bg-primary-600 dark:hover:bg-primary-700 focus:outline-none dark:focus:ring-primary-800">Get started</a> -->
           
        </div>
          {/if}
          
         
      </div>
  </nav>
</header>
<!--
  This example requires some changes to your config:
  
  ```
  // tailwind.config.js
  module.exports = {
    // ...
    plugins: [
      // ...
      require('@tailwindcss/aspect-ratio'),
    ],
  }
  ```
-->
<input id="input_file" hidden type="file" on:change={onselect}/>

{#if files.length == 0}
<div class="text-center mt-24">
  <svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor" aria-hidden="true">
    <path vector-effect="non-scaling-stroke" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 13h6m-3-3v6m-9 1V7a2 2 0 012-2h6l2 2h6a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z" />
  </svg>
  <h3 class="mt-2 text-sm font-semibold text-gray-900">No projects</h3>
  <p class="mt-1 text-sm text-gray-500">Get started by creating a new project.</p>
  <div class="mt-6">
    
    <button on:click={e => {
     document.getElementById("input_file").click() 
    }} class="inline-flex items-center rounded-md bg-indigo-600 px-3 py-2 text-sm font-semibold text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600">
      <svg class="-ml-0.5 mr-1.5 h-5 w-5" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
        <path d="M10.75 4.75a.75.75 0 00-1.5 0v4.5h-4.5a.75.75 0 000 1.5h4.5v4.5a.75.75 0 001.5 0v-4.5h4.5a.75.75 0 000-1.5h-4.5v-4.5z" />
      </svg>
      Upload File
    </button>
  </div>
</div>
{:else}
<div class="p-4">
  <ul role="list" class="grid grid-cols-2 gap-x-4 gap-y-8 sm:grid-cols-3 sm:gap-x-6 lg:grid-cols-4 xl:gap-x-8">
    <li class="relative">
      <div class="group aspect-h-4 aspect-w-8 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
        <button on:click={e => {
          document.getElementById("input_file").click() 
        }} class="w-full h-full bg-indigo-100 flex items-center justify-center text-indigo-600">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-16 h-16">
            <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
          </svg>
          
          
        </button>
       
      </div>
      
    </li>
    {#each files as icpsummary}
    <li class="relative">
      <div class="group aspect-h-7 aspect-w-10 block w-full overflow-hidden rounded-lg bg-gray-100 focus-within:ring-2 focus-within:ring-indigo-500 focus-within:ring-offset-2 focus-within:ring-offset-gray-100">
        <div class="w-full h-full bg-indigo-100 flex items-center justify-center text-indigo-600">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-16 h-16">
            <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 14.25v-2.625a3.375 3.375 0 0 0-3.375-3.375h-1.5A1.125 1.125 0 0 1 13.5 7.125v-1.5a3.375 3.375 0 0 0-3.375-3.375H8.25m2.25 0H5.625c-.621 0-1.125.504-1.125 1.125v17.25c0 .621.504 1.125 1.125 1.125h12.75c.621 0 1.125-.504 1.125-1.125V11.25a9 9 0 0 0-9-9Z" />
          </svg>
          
        </div>
        <button type="button" class="absolute inset-0 focus:outline-none">
          <span class="sr-only">View details for IMG_4985.HEIC</span>
        </button>
      </div>
      <p class="pointer-events-none mt-2 block truncate text-sm font-medium text-gray-900">{icpsummary.name}</p>
      <p class="pointer-events-none block text-sm font-medium text-gray-500">{formatBytes(Number(icpsummary.size))}</p>
    </li>
    {/each}
    
  
    <!-- More files... -->
  </ul>
</div>
{/if}

  
</main>
