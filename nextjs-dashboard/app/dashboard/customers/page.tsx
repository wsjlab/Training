import { createClient } from 'app/utils/supabase/server'
import { cookies } from 'next/headers'
import React from 'react'

export default async function Page() {
  const cookieStore = await cookies()
  const supabase = createClient(cookieStore)

  const { data: auteurs } = await supabase.from('auteurs').select('nom')

  return (
    <ul>
      {auteurs?.map((auteur, index) => (
        <li key={index}>{auteur.nom}</li>
      ))}
    </ul>
  )
}



/*export default function Page(){
    return <p>Customers Page</p>

}*/
