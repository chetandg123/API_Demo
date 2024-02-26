<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>solution</name>
   <tag></tag>
   <elementGuidId>3c77c066-bc9c-437d-b210-9a1ced1d4beb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;context\&quot;: {\n        \&quot;domain\&quot;: \&quot;onest:work-opportunities\&quot;,\n        \&quot;action\&quot;: \&quot;select\&quot;,\n        \&quot;version\&quot;: \&quot;1.1.0\&quot;,\n        \&quot;bap_id\&quot;: \&quot;wo-ps-bap-network.onest.network\&quot;,\n        \&quot;bap_uri\&quot;: \&quot;https://wo-ps-bap-network.onest.network/\&quot;,\n        \&quot;location\&quot;: {\n            \&quot;country\&quot;: {\n                \&quot;name\&quot;: \&quot;India\&quot;,\n                \&quot;code\&quot;: \&quot;BH\&quot;\n            },\n            \&quot;city\&quot;: {\n                \&quot;name\&quot;: \&quot;Bangalore\&quot;,\n                \&quot;code\&quot;: \&quot;std:080\&quot;\n            }\n        },\n        \&quot;bpp_id\&quot;: \&quot;dev-onest.tibilprojects.com\&quot;,\n        \&quot;bpp_uri\&quot;: \&quot;https://dev-onest.tibilprojects.com/protocol-network\&quot;,\n        \&quot;transaction_id\&quot;: \&quot;${randomUID}\&quot;,\n        \&quot;message_id\&quot;: \&quot;46a27d53-3335-4136-b6f1-0065c70269e7\&quot;,\n        \&quot;timestamp\&quot;: \&quot;2023-02-06T09:55:41.161Z\&quot;\n    },\n    \&quot;message\&quot;: {\n        \&quot;order\&quot;: {\n            \&quot;provider\&quot;: {\n                \&quot;id\&quot;: \&quot;1\&quot;\n            },\n            \&quot;items\&quot;: [\n                {\n                    \&quot;id\&quot;: \&quot;1\&quot;\n                }\n            ]\n        }\n    }  \n}\n&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>ca49384d-6547-449d-a9bb-a5f93713e73e</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://wo-ps-bap-client.onest.network/select</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'1'</defaultValue>
      <description></description>
      <id>2162510b-2fda-4739-ab82-35fd73503fb8</id>
      <masked>false</masked>
      <name>randomUID</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

def sluper = new JsonSlurper()

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

println(&quot;../API RESPONSE code : &quot;+ response)

def result = sluper.parseText(response.getResponseBodyContent())

println(&quot;../API RESPONSE information: &quot;+ result)




WS.verifyResponseStatusCode(response, 200)

assertThat(response.getStatusCode()).isEqualTo(200)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
