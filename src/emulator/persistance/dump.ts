import { type Save } from '../constants'

export const dumpToFile = ({
  data,
  state,
  metadata: { cartTitle, createdAt, lastAccessed }
}: Omit<Save, 'id'>): File => {
  const $createAt = createdAt?.getTime() ?? 0
  const $lastAccessed = lastAccessed?.getTime() ?? 0
  const $titleSection = new TextEncoder().encode(cartTitle) //变长内容
  const $dataSection = data // 变长内容
  const ret = new ArrayBuffer(8 + 24 + $titleSection.byteLength + $dataSection.byteLength) // 标题长度字段4字节 数据长度字段4字节 元数据段24字节 标题段变长 数据段变长
  const head = new Uint32Array(ret, 0, 2)
  head[0] = $titleSection.byteLength
  head[1] = $dataSection.byteLength
  const metaSection = new BigUint64Array(ret, 8, 3)
  metaSection[0] = BigInt(state)
  metaSection[1] = BigInt($createAt)
  metaSection[2] = BigInt($lastAccessed)
  const titleSection = new Uint8Array(ret, 32, $titleSection.byteLength)
  titleSection.set($titleSection)
  const dataSection = new Uint8Array(ret, 32 + $titleSection.byteLength, $dataSection.byteLength)
  dataSection.set($dataSection)
  const filename =
    lastAccessed === undefined
      ? `${cartTitle}.ygb`
      : `${cartTitle}-${lastAccessed.getFullYear().toString().padStart(4, '0')}-${lastAccessed.getMonth().toString().padStart(2, '0')}-${lastAccessed.getDay().toString().padStart(2, '0')}-${lastAccessed.getHours().toString().padStart(2, '0')}-${lastAccessed.getMinutes().toString().padStart(2, '0')}-${lastAccessed.getSeconds().toString().padStart(2, '0')}.ygb`
  return new File([ret], filename)
}

export const loadFromFile = async (file: File): Promise<Omit<Save, 'id'>> => {
  // 读取文件的 ArrayBuffer
  const arrayBuffer = await file.arrayBuffer()

  // 解析头部信息（标题长度和数据长度）
  const head = new Uint32Array(arrayBuffer, 0, 2)
  const titleLength = head[0]
  const dataLength = head[1]

  // 解析元数据段（状态、创建时间、最后访问时间）
  const metaSection = new BigUint64Array(arrayBuffer, 8, 3)
  const state = Number(metaSection[0])
  const createdAt = metaSection[1] == 0n ? undefined : new Date(Number(metaSection[1]))
  const lastAccessed = metaSection[2] == 0n ? undefined : new Date(Number(metaSection[2]))

  // 解析标题段
  const titleSection = new Uint8Array(arrayBuffer, 32, titleLength)
  const cartTitle = new TextDecoder().decode(titleSection)

  // 解析数据段
  const dataSection = new Uint8Array(arrayBuffer, 32 + titleLength, dataLength)
  const data = dataSection

  // 返回解析后的对象
  return {
    data,
    state,
    metadata: {
      cartTitle,
      createdAt,
      lastAccessed
    }
  }
}
