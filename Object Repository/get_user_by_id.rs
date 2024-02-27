<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>get_user_by_id</name>
   <tag></tag>
   <elementGuidId>314b21b3-47ac-4b03-bb22-f64af1821b70</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.6.8</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${api_url}/users/2</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.api_url</defaultValue>
      <description></description>
      <id>1cf1b878-4709-4631-ac86-e34e237299f4</id>
      <masked>false</masked>
      <name>api_url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager
import groovy.json.JsonSlurper as JsonSlurper

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

def sluper = new JsonSlurper()
def result = sluper.parseText(response.getResponseBodyContent())


WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)

println('API Response is: '+ result['data']['id'])

if(result['data']['id'] == 2)
{
    println('User 2 : API Response is fetched Successfully...')
	assert true	
}
else {
	println('Error!: User 2: API Response is not Fetched ')
	assert false
}
if(result['data']['first_name'] == 'Janet')
	{
		println('First Name is Matched ')
		assert true
	}
else {
		println('Error!: First Name is not matched ')
		assert false
}
if(result['data']['last_name'] == 'Weaver')
	{
		println('Last Name is Matched ')
		assert true
	}
else {
	println('Error!: Last Name is not matched ')
	assert false
}

if(result['data']['email'] == 'janet.weaver@reqres.in')
	{
		println('Email id is Matched ')
		assert true
	}
else {
	println('Error!: Email id is not matched ')
	assert false
}



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
