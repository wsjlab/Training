/* import { createClient } from 'app/utils/supabase/server'
import { cookies } from 'next/headers'
import React from 'react'

export default async function Page() {
  const cookieStore = await cookies()
  const supabase = createClient(cookieStore)

  const { data: auteurs } = await supabase.from('auteurs').select('*')
  console.log(auteurs)
  return (
    <ul>
        {auteurs?.map((auteur, index) => (
          <li key={index}>
            {Object.entries(auteur).map(([key, value]) => (
              <div key={key}>
                <strong>{key}:</strong> {value}
              </div>
            ))}
          </li>
        ))}
    </ul>
  )
} */



/*export default function Page(){
    return <p>Customers Page</p>

}*/

import { Payment, columns } from "./columns"
import { DataTable } from "./data-table"
import {Card,CardContent, CardDescription,CardFooter,CardHeader,CardTitle} from "@/components/ui/card"


async function getData(): Promise<Payment[]> {
  // Fetch data from your API here.
  return [
    {
      id: "728ed52f",
      amount: 100,
      status: "pending",
      email: "m@example.com",
    },
    {
      id: "728ed52f",
      amount: 100,
      status: "pending",
      email: "m@example.com",
    },
    {
      id: "489e1d42",
      amount: 125,
      status: "processing",
      email: "example@gmail.com",
    },
    
    // ...
  ]
}

export default async function DemoPage() {
  const data = await getData()

  return (

  
  <div className="container mx-auto py-10">
    <Card className="sd:w-full xl:w-1/3 w-3/4 h-1/2 mx-auto">
      <CardHeader>
        <CardTitle className="text-center">COVER OF THE WEEK</CardTitle>
        <CardDescription>Issue 2025 #01</CardDescription>
      </CardHeader>
      <CardContent>
        <img src="/WSJ_Issue_2025_01_Cover.jpg" alt="WSJ issue 2025 01" className="w-full h-auto rounded-lg" />
        <p className="text-center">Card Content</p>
      </CardContent>
      <CardFooter>
        <p>Card Footer</p>
      </CardFooter>
    </Card>

      <DataTable columns={columns} data={data} />
  </div>
  )
}
